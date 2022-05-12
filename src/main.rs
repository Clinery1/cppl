use std::{
    fs::read_to_string,
};
pub use grammar::*;


mod grammar;


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
    let data=read_to_string("example.cppl").unwrap();
    //let data=read_to_string("hello_world.cppl").unwrap();
    peg_start(&data);
    let parsed=grammar::parse_module(&data);
    peg_end();
    //println!("Data: `{}`",data);
    println!("Parsed: {:#?}",parsed.unwrap());
}
fn peg_start(data:&str) {
    if cfg!(feature="trace") {
        println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]",data);
    }
}
fn peg_end() {
    if cfg!(feature="trace") {
        println!("[PEG_TRACE_STOP]");
    }
}
