use std::{
    fs::read_to_string,
    time::Instant,
};
use cppl_parser::parse;
use cppl_error::*;
use cppl_ast::{
    analyze,
    AnalysisResults,
};


fn main() {
    let start=Instant::now();
    let filename="example2.cppl";
    let source=read_to_string(filename).unwrap();
    match parse(filename,&source) {
        Ok(parsed)=>{
            match analyze(filename,parsed) {
                Ok((refined,AnalysisResults{warnings,..}))=>{
                    let warn_count=warnings.len();
                    for warning in warnings {
                        println!("{}",ContextualError::from((source.as_str(),warning)));
                    }
                    println!("{} generated {} warnings",filename,warn_count);
                    println!("Refined scopes: {:#?}",refined);
                },
                Err(AnalysisResults{errors,warnings})=>{
                    let warn_count=warnings.len();
                    let err_count=errors.len();
                    for warning in warnings {
                        println!("{}",ContextualError::from((source.as_str(),warning)));
                    }
                    for err in errors {
                        println!("{}",ContextualError::from((source.as_str(),err)));
                    }
                    println!("{} generated {} warnings",filename,warn_count);
                    println!("{} generated {} errors",filename,err_count);
                },
            }
        },
        Err(e)=>println!("{}",ContextualError::from((source.as_str(),e))),
    }
    let elapsed=start.elapsed();
    println!("Elapsed time: {:?}",elapsed);
}
