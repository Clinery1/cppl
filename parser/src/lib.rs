use lalrpop_util::{
    ParseError,
    lalrpop_mod,
};
use cppl_error::*;
use cppl_ast::raw::Statement;
use cppl_lexer::*;


lalrpop_mod!(pub parser);


pub fn parse<'input>(filename:&'input str,source:&'input str)->Result<Vec<Statement<'input>>,Error<'input,String>> {
    let tokens=TokenIterator::new(&source,filename,true);
    return parser::AllParser::new().parse(filename,tokens).map_err(|e|convert_error(e,filename));
}
pub(crate) fn parse_char<'input>(s:String,filename:&'input str,start:Location,end:Location)->Result<char,Error<'input,&'static str>> {
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
fn convert_error<'input>(e:ParseError<Location,Token<'input>,Error<'input,&'input str>>,filename:&'input str)->Error<'input,String> {
    use ParseError::*;
    let level=ErrorLevel::ParseError;
    match e {
        InvalidToken{location}=>Error {
            start:location,
            end:location,
            filename,
            level,
            reason:"(internal error) unknown token".into(),
        },
        UnrecognizedEOF{location,..}=>{
            Error {
                start:location,
                end:location,
                filename,
                level,
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
            Error {
                start,
                end,
                filename,
                level,
                reason,
            }
        },
        ExtraToken{token:(start,token,end)}=>Error {
            start,
            end,
            filename,
            level,
            reason:format!("Unexpected {}",token),
        },
        User{error:Error{start,end,filename,level,reason}}=>Error {
            start,
            end,
            filename,
            level,
            reason:reason.to_string(),
        },
    }
}
