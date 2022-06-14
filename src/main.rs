#[macro_use] extern crate lalrpop_util;


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


pub struct Error<'a,T:Display> {
    pub filename:&'a str,
    pub line:usize,
    pub column:usize,
    pub line_data:&'a str,
    pub reason:T,
    pub level:ErrorLevel,
}
impl<'a,T:Display> Debug for Error<'a,T> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        write!(f,"{}: {} at {}:{}",self.level,self.reason,self.line+1,self.column+1)
    }
}
impl<'a,T:Display+Debug> Display for Error<'a,T> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        writeln!(f,"{}: {}",self.level,self.reason)?;
        writeln!(f,"    ╭╴{}:{}:{}",self.filename,self.line+1,self.column+1)?;
        writeln!(f,"    │")?;
        writeln!(f,"{:>4}│ {}",self.line+1,self.line_data)?;
        write!(f,  "    ╰─")?;
        for _ in 0..self.column {
            write!(f,"─")?;
        }
        write!(f,"╯")?;
        return Ok(());
    }
}


fn main() {
    let filename="example2.cppl";
    let data=read_to_string(filename).unwrap();
    let tokens=TokenIterator::new(&data,filename);
    let parsed=parser::AllParser::new().parse(tokens);
    println!("{:#?}",parsed);
}
