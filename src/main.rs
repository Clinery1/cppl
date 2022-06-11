use logos::Logos;
use std::{
    fs::read_to_string,
};
use lexer::*;


mod lexer;
mod parser;


fn main() {
    let filename="example.cppl";
    let data=read_to_string(filename).unwrap();
    /*
    let data=r###"fn main_123[] {
    print(r#"Hello, World!"#)
    // Comment!
    c:='\x6d'
}"###;
    */
    let tokens=Token::lexer(&data).collect::<Vec<_>>();
    println!("{:#?}",tokens);
}
