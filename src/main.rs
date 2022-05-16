use std::{
    fs::read_to_string,
};
use lexer::*;


mod error;
mod lexer;
mod parser;


#[derive(Debug)]
pub enum Error {
    NoBoolConstraint,
    NoByteConstraint,
    NoUByteConstraint,
    NoUIntConstraint,
    NoIntConstraint,
    NoLongIntConstraint,
    NoLongUIntConstraint,
    NoSingleFloatConstraint,
    NoDoubleFloatConstraint,
    RequestPathCollapse,
    UnknownConstraint(String),
}


fn main() {
    let filename="hello_world.cppl";
    let data=read_to_string(filename).unwrap();
    let tokens=TokenList::new(&data);
    if let Ok(tokens)=tokens {
        for token in tokens.0.iter() {
            match token {
                Token::LineComment(_)|
                    Token::BlockComment(_)|
                    Token::Newline(_)|
                    Token::Space(_)|
                    Token::Tab(_)=>{},
                _=>{
                    println!("{:?}",token);
                },
            }
        }
        //println!("Recreated source: `{}`",tokens.recreate_source());
    } else {
        let err=tokens.unwrap_err();
        err.print_with_context(filename,&data);
    }
}
