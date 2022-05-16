use std::fmt::{
    Formatter,
    Display,
    Result as FmtResult,
};


#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(usize)]
pub enum ErrorType {
    MultipleDecimal,
    UnclosedBlockComment,
    UnexpectedEOFString,
    ExpectedUnicodeEscapeStart(char),
    ExpectedUnicodeEscapeEnd(char),
    ExpectedUnicodeEscape(char),
    ExpectedAsciiEscape(char),
    UnknownEscape(char),
    UnicodeEscapeOutOfRange(u32),
    AsciiEscapeOutOfRange(u32),
}
impl ErrorType {
    pub fn id(&self)->usize {
        use ErrorType::*;
        match self {
            MultipleDecimal=>0,
            UnclosedBlockComment=>1,
            UnexpectedEOFString=>2,
            ExpectedUnicodeEscapeStart(_)=>3,
            ExpectedUnicodeEscapeEnd(_)=>4,
            ExpectedUnicodeEscape(_)=>5,
            ExpectedAsciiEscape(_)=>6,
            UnknownEscape(_)=>7,
            UnicodeEscapeOutOfRange(_)=>8,
            AsciiEscapeOutOfRange(_)=>9,
        }
    }
}
impl Display for ErrorType {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use ErrorType::*;
        match self {
            MultipleDecimal=>write!(f,"Multiple decimals in number"),
            UnclosedBlockComment=>write!(f,"Unclosed block comment"),
            UnexpectedEOFString=>write!(f,"Expected `\"`, got EOF"),
            ExpectedUnicodeEscapeStart(c)=>write!(f,"Expected '{{' in unicode escape, but got '{}'",c),
            ExpectedUnicodeEscapeEnd(c)=>write!(f,"Expected '}}' in unicode escape, but got '{}'",c),
            ExpectedUnicodeEscape(c)=>write!(f,"Expected unicode escape sequence, but got '{}'",c),
            ExpectedAsciiEscape(c)=>write!(f,"Expected ascii escape sequence, but got '{}'",c),
            UnknownEscape(c)=>write!(f,"Unknown escape char: '{}'",c),
            UnicodeEscapeOutOfRange(num)=>write!(f,"Unicode escape out of range. Expected range 0x0..=0x{:X}, but got {:X}",char::MAX as u32,num),
            AsciiEscapeOutOfRange(num)=>write!(f,"Ascii escape out of range. Expected range 0x0..=0x7F, but got {:X}",num),
        }
    }
}


#[derive(Clone,Debug,PartialEq)]
pub struct Error {
    pub line:usize,
    pub column:usize,
    pub error:ErrorType,
}
impl Error {
    pub fn print_with_context(&self,filename:impl Display,contents:&str) {
        let code_line=contents.lines().nth(self.line-1).unwrap();
        let cl_char_count=code_line.chars().count();
        let code_line_trimmed=code_line.trim();
        let clt_char_count=code_line_trimmed.chars().count();
        println!("Error: {}",self.error);
        println!("    ╭╴{}:{}:{}",filename,self.line,self.column);
        println!("    │");
        let mut line_str=self.line.to_string();
        while line_str.len()<4 {line_str.push(' ')}
        print!("{}│ ",line_str);
        let code_line_removed=cl_char_count-clt_char_count;
        let to_add="… ";
        let mut to_add_len=0;
        if code_line_removed>0 {
            to_add_len+=to_add.chars().count();
            print!("{}",to_add);
        }
        println!("{}",code_line_trimmed);
        print!("    ╰");
        for _ in 0..((self.column+to_add_len)-code_line_removed) {
            print!("─");
        }
        println!("╯");
    }
}
impl Display for Error {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        write!(f,"Error #{} at ({}:{}) {}",self.error.id(),self.line,self.column,self.error)
    }
}
