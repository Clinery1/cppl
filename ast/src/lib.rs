use cppl_error::{
    Error,
};


pub mod raw;
pub mod refined;
mod static_analysis;


#[derive(Default)]
pub struct AnalysisResults<'input> {
    pub warnings:Vec<Error<'input,String>>,
    pub errors:Vec<Error<'input,String>>,
}


pub fn analyze<'input>(filename:&'input str,mut stmts:Vec<raw::Statement<'input>>)->Result<(refined::Scopes<'input>,AnalysisResults<'input>),AnalysisResults<'input>> {
    let mut ret=AnalysisResults::default();
    match static_analysis::verify(filename,&mut stmts) {
        Ok(mut w)=>ret.warnings.append(&mut w),
        Err([mut e,mut w])=>{
            ret.warnings.append(&mut w);
            ret.errors.append(&mut e);
            return Err(ret);
        },
    }
    match static_analysis::refine(stmts,filename) {
        Ok(re)=>{
            return Ok((re,ret));
        },
        Err(e)=>{
            ret.errors.push(e);
            return Err(ret);
        },
    }
}
fn is_camel_case(s:&str)->bool {
    let mut chars=s.chars();
    while let Some('_')=chars.next() {}
    match chars.next().unwrap_or('_') {
        c if c.is_ascii_uppercase()=>chars.all(|c|c!='_'),
        _=>false,
    }
}
fn is_snake_case(s:&str)->bool {
    s.chars().all(|c|c.is_ascii_lowercase()||c=='_'||c.is_ascii_digit())
}
fn is_screaming_snake_case(s:&str)->bool {
    s.chars().all(|c|c.is_ascii_uppercase()||c=='_'||c.is_ascii_digit())
}
