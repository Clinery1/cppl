use std::{
    fmt::{
        Display,
        Debug,
        Result as FmtResult,
        Formatter,
    },
};


#[derive(Copy,Clone,Debug)]
pub enum ErrorLevel {
    Warning,
    LexError,
    ParseError,
    Verification,
}
impl Display for ErrorLevel {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use ErrorLevel::*;
        match self {
            Warning=>write!(f,"Warning"),
            LexError=>write!(f,"Lex error"),
            ParseError=>write!(f,"Parse error"),
            Verification=>write!(f,"Verification error"),
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
impl<'source,T:Display> Error<'source,T> {
    pub fn new_verif<R:Into<T>>(filename:&'source str,start:Location,end:Location,reason:R)->Self {
        Error{filename,start,end,reason:reason.into(),level:ErrorLevel::Verification}
    }
    pub fn new_warning<R:Into<T>>(filename:&'source str,start:Location,end:Location,reason:R)->Self {
        Error{filename,start,end,reason:reason.into(),level:ErrorLevel::Warning}
    }
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
#[derive(Copy,Clone,Debug,PartialEq,Default)]
pub struct Location {
    pub line_start_index:usize,
    pub index:usize,
    pub line:usize,
    pub column:usize,
}
