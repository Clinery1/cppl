// hello
use lalrpop_util::{
    ParseError,
    lalrpop_mod,
};
use std::{
    fmt::{
        Display,
        Debug,
        Result as FmtResult,
        Formatter,
    },
    fs::read_to_string,
};
use lexer::*;


mod lexer;
mod ast;
lalrpop_mod!(pub parser);


#[derive(Copy,Clone,Debug)]
pub enum ErrorLevel {
    Warning,
    LexError,
    ParseError,
}
impl Display for ErrorLevel {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use ErrorLevel::*;
        match self {
            Warning=>write!(f,"Warning"),
            LexError=>write!(f,"Lex error"),
            ParseError=>write!(f,"Parse error"),
        }
    }
}


#[derive(Debug)]
pub struct ContextualError<'source,T:Display+Debug> {
    pub filename:&'source str,
    pub start:Location,
    pub end:Location,
    pub reason:T,
    pub level:ErrorLevel,
    pub source:&'source str,
}
impl<'source,T:Display+Debug> From<(&'source str,Error<'source,T>)> for ContextualError<'source,T> {
    fn from((source,error):(&'source str,Error<'source,T>))->Self {
        let Error{filename,start,end,reason,level}=error;
        return ContextualError {
            filename,
            start,
            end,
            reason,
            level,
            source,
        };
    }
}
impl<'source>  ContextualError<'source,String> {
    fn from_parse_error<T:Display>(e:ParseError<Location,Token<'source>,Error<'source,T>>,source:&'source str,filename:&'source str)->Self {
        use ParseError::*;
        let level=ErrorLevel::ParseError;
        match e {
            InvalidToken{location}=>ContextualError {
                start:location,
                end:location,
                filename,
                level,
                source,
                reason:"(internal error) unknown token".into(),
            },
            UnrecognizedEOF{location,..}=>{
                ContextualError {
                    start:location,
                    end:location,
                    filename,
                    level,
                    source,
                    reason:"Unexpected EOF".into(),
                }
            },
            UnrecognizedToken{token:(start,token,end),expected}=>{
                let mut reason=String::from("Expected: ");
                let final_i=expected.len()-1;
                for (i,option) in expected.into_iter().enumerate() {
                    reason.push_str(&option);
                    if i==final_i {
                        reason.push(' ');
                    } else {
                        reason.push_str(", ");
                    }
                }
                reason.push_str("found ");
                let token=token.to_string();
                reason.push_str(&token);
                ContextualError {
                    start,
                    end,
                    filename,
                    level,
                    source,
                    reason,
                }
            },
            ExtraToken{token:(start,token,end)}=>ContextualError {
                start,
                end,
                filename,
                level,
                source,
                reason:format!("Unexpected {}",token),
            },
            User{error:Error{start,end,filename,level,reason}}=>ContextualError {
                start,
                end,
                filename,
                level,
                source,
                reason:reason.to_string(),
            },
        }
    }
}
impl<'source,T:Display+Debug> Display for ContextualError<'source,T> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        if self.start.line>self.end.line||self.start.index>self.end.index {
            panic!("Invalid error: {:#?}",self);
        }
        if self.start.line!=self.end.line {
            let end=self
                .source[self.end.index..]
                .chars()
                .enumerate()
                .find(|(_,c)|*c=='\n')
                .map_or(self.end.index,|(i,_)|i+self.end.index);
            let lines=&self.source[self.start.line_start_index..end];
            let end_line=format!("{}",self.end.line);
            let num_width=end_line.len();
            writeln!(f,"{}: {}",self.level,self.reason)?;
            writeln!(f,"{:>width$}╭╴{}:[{}:{}]..[{}:{}]","",self.filename,self.start.line+1,self.start.column+1,self.end.line+1,self.end.column+1,width=num_width)?;
            writeln!(f,"{:>width$}│","",width=num_width)?;
            let mut start_line=self.start.line+1;
            let mut last_line_len=0;
            for line in lines.lines() {
                writeln!(f,"{:>2$}│ {}",start_line,line,num_width)?;
                start_line+=1;
                last_line_len=line.len();
            }
            write!(f,  "{:>1$}╰─","",num_width)?;
            for _ in 0..last_line_len {
                write!(f,"─")?;
            }
        } else {
            let end=self
                .source[self.end.index..]
                .chars()
                .enumerate()
                .find(|(_,c)|*c=='\n')
                .map_or(0,|(i,_)|i)+self.end.index;
            let line=&self.source[self.start.line_start_index..end];
            let end_line=format!("{}",self.end.line+1);
            let num_width=end_line.len();
            writeln!(f,"{}: {}",self.level,self.reason)?;
            if self.start.column==self.end.column {
                writeln!(f,"{:>width$} ╭╴{}:{}:{}","",self.filename,self.start.line+1,self.start.column+1,width=num_width)?;
            } else {
                writeln!(f,"{:>width$} ╭╴{}:{}:[{}..{}]","",self.filename,self.start.line+1,self.start.column+1,self.end.column,width=num_width)?;
            }
            writeln!(f,"{:>width$} │","",width=num_width)?;
            writeln!(f,"{:>2$} │ {}",end_line,line,num_width)?;
            write!(f,  "{:>1$} ╰─","",num_width)?;
            for _ in 0..self.start.column {
                write!(f,"─")?;
            }
            for _ in self.start.column..self.end.column.saturating_sub(1) {
                write!(f,"┴")?;
            }
            write!(f,"╯")?;
        }
        return Ok(());
    }
}
pub struct Error<'source,T:Display> {
    pub filename:&'source str,
    pub start:Location,
    pub end:Location,
    pub reason:T,
    pub level:ErrorLevel,
}
impl<'source,T:Display> Debug for Error<'source,T> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        write!(f,"{}: {} in file `{}` at `",self.level,self.reason,self.filename)?;
        if self.start.line!=self.end.line {
            write!(f,"[{}:{}]..[{}:{}]",self.start.line+1,self.start.column+1,self.end.line+1,self.end.column+1)?;
        } else {
            if self.start.column!=self.end.column {
                write!(f,"{}:[{}..{}]",self.start.line+1,self.start.column+1,self.end.column+1)?;
            } else {
                write!(f,"{}:{}",self.start.line+1,self.start.column+1)?;
            }
        }
        write!(f,"`")?;
        return Ok(());
    }
}
impl<'source,T:Display> Display for Error<'source,T> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        write!(f,"{:?}",self)
    }
}
pub struct TokenWrapper<'input>(TokenIterator<'input>);
impl<'input> Iterator for TokenWrapper<'input> {
    type Item=Result<(Location,Token<'input>,Location),Error<'input,&'static str>>;
    fn next(&mut self)->Option<Self::Item> {
        let token=self.0.next()?;
        println!("Token: {:?}",token);
        return Some(token);
    }
}


fn main() {
    let filename="example2.cppl";
    let source=read_to_string(filename).unwrap();
    let tokens=TokenWrapper(TokenIterator::new(&source,filename,true));
    let parsed=parser::AllParser::new().parse(filename,tokens);
    match parsed {
        Ok(parsed)=>println!("{:#?}",parsed),
        Err(e)=>{
            let err=ContextualError::from_parse_error(e,&source,filename);
            println!("{}",err);
        },
    }
}
pub fn parse_char<'source>(s:String,filename:&'source str,start:Location,end:Location)->Result<char,Error<'source,&'static str>> {
    let mut start=start;
    start.column+=1;
    let mut end=end;
    end.column-=1;
    let level=ErrorLevel::ParseError;
    match s.as_str() {
        c if c.starts_with('\\')=>{
            let c=&c[1..];
            match c.len() {
                1=>{
                    match c {
                        "n"=>return Ok('\n'),
                        "t"=>return Ok('\n'),
                        "r"=>return Ok('\n'),
                        "\\"=>return Ok('\\'),
                        "\\'"=>return Ok('\''),
                        "0"=>return Ok('\0'),
                        _=>{},
                    }
                },
                3=>{
                    if c.starts_with('x') {
                        let hex=&c[1..];
                        if !hex.chars().all(|c|c.is_ascii_hexdigit()) {
                            return Err(Error {
                                filename,
                                start,
                                end,
                                level,
                                reason:"invalid ASCII escape sequence",
                            });
                        }
                        let num=u8::from_str_radix(hex,16).unwrap();
                        return Ok(num as char);
                    }
                },
                _=>{
                    if c.starts_with('u') {
                        let c=&c[1..];
                        if !c.starts_with('{') {
                            start.column+=2;
                            return Err(Error {
                                filename,
                                start,
                                end:start,
                                level,
                                reason:"expected \"{\" got hex number"
                            });
                        }
                        if !c.ends_with('}') {
                            start.column+=c.len()-1;
                            return Err(Error {
                                filename,
                                start,
                                end:start,
                                level,
                                reason:"expected \"}\" got token `'`",
                            });
                        }
                        let hex=&c[1..c.len()-1];
                        start.column+=3;
                        end.column-=1;
                        let error=Error {
                            filename,
                            start,
                            end,
                            level,
                            reason:"unicode escape sequence must be between 0x0 and 0xD7FF or 0xE000 and 0x10FFFF including these values",
                        };
                        if !hex.chars().all(|c|c.is_ascii_hexdigit()) {
                            return Err(Error {
                                filename,
                                start,
                                end,
                                level,
                                reason:"invalid hex data",
                            });
                        }
                        if hex.len()>6 {
                            return Err(error);
                        }
                        let num=u32::from_str_radix(hex,16).unwrap();
                        return char::from_u32(num).ok_or(error);
                    }
                },
            }
            return Err(Error {
                filename,
                start,
                end,
                level,
                reason:"invalid escape sequence",
            });
        },
        _=>{
            if s.len()!=1 {
                return Err(Error {
                    filename,
                    start,
                    end,
                    level,
                    reason:"`char` can only have one character, or a valid escape sequence",
                });
            }
            return Ok(s.chars().next().unwrap());
        },
    }
}
