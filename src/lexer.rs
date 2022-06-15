use logos::{
    Logos,
    Lexer,
};
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};
use crate::{
    Error,
    ErrorLevel,
};


#[derive(Logos,Debug,PartialEq,Clone)]
pub enum Token<'input> {
    #[token("fn", |_|Keyword::Function)]
    #[token("type", |_|Keyword::Type)]
    #[token("interface", |_|Keyword::Interface)]
    #[regex("pub", |_|Keyword::Pub)]
    #[regex("mut", |_|Keyword::Mut)]
    #[token("impl", |_|Keyword::Impl)]
    #[token("import", |_|Keyword::Import)]
    #[token("for", |_|Keyword::For)]
    #[token("while", |_|Keyword::While)]
    #[token("loop", |_|Keyword::Loop)]
    #[token("break", |_|Keyword::Break)]
    #[token("return", |_|Keyword::Return)]
    #[token("continue", |_|Keyword::Continue)]
    #[token("match", |_|Keyword::Match)]
    #[token("enum", |_|Keyword::Enum)]
    #[token("module", |_|Keyword::Module)]
    #[token("this", |_|Keyword::This)]
    Keyword(Keyword),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*",slice)]
    Word(&'input str),
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("::")]
    Associated,
    #[token(";")]
    SemiColon,
    #[token("(")]
    ParenthesisStart,
    #[token(")")]
    ParenthesisEnd,
    #[token("[")]
    BracketStart,
    #[token("]")]
    BracketEnd,
    #[token("{")]
    BraceStart,
    #[token("}")]
    BraceEnd,
    #[token(":=")]
    Decl,
    #[token(">=")]
    GreaterEqual,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token("<")]
    Less,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Assign,
    #[token("-")]
    Dash,
    #[token(".")]
    Dot,
    #[token("...")]
    Etc,
    #[token("|")]
    Union,
    #[token("+")]
    Add,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("&")]
    And,
    #[token("!")]
    Not,
    #[token("'")]
    SingleQuote,
    #[token("\\")]
    Backslash,
    #[regex("r#*\"",parse_raw_string)]
    #[regex("\"[^\"]*\"",fix_string)]
    String(&'input str),
    #[regex("\n+")]
    Newline,
    #[regex("[0-9][0-9_]*",slice)]
    Number(&'input str),
    #[regex("[0-9]+\\.[0-9]+",slice)]
    #[regex("\\.[0-9]+",slice)]
    #[regex("[0-9]+\\.",slice)]
    Float(&'input str),
    #[regex("///[^\n]*",line_doc_comment_fix)]
    #[regex("/\\*\\*[^**/]*\\*\\*/",block_doc_comment_fix)]
    DocComment(&'input str),
    #[regex("[ \t]+", logos::skip)]
    //#[regex("[ \t]+")]
    //Whitespace,
    #[regex("//[^\n]*",logos::skip)]
    #[regex("/\\*[^*/]*\\*/",logos::skip)]
    Comment,
    #[error]
    Error,
}
impl<'source> Display for Token<'source> {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use Token::*;
        match self {
            Keyword(kw)=>write!(f,"keyword: `{}`",kw),
            Word(s)=>write!(f,"word: `{}`",s),
            Comma=>write!(f,"token: `,`"),
            Colon=>write!(f,"token: `:`"),
            Associated=>write!(f,"token: `::`"),
            SemiColon=>write!(f,"token: `;`"),
            ParenthesisStart=>write!(f,"token: `(`"),
            ParenthesisEnd=>write!(f,"token: `)`"),
            BracketStart=>write!(f,"token: `]`"),
            BracketEnd=>write!(f,"token: `]`"),
            BraceStart=>write!(f,"token: `{{`"),
            BraceEnd=>write!(f,"token: `}}`"),
            Decl=>write!(f,"token: `:=`"),
            GreaterEqual=>write!(f,"token: `>=`"),
            Greater=>write!(f,"token: `>`"),
            LessEqual=>write!(f,"token: `<=`"),
            Less=>write!(f,"token: `<`"),
            Equal=>write!(f,"token: `==`"),
            NotEqual=>write!(f,"token: `!="),
            Assign=>write!(f,"token: `=`"),
            Dash=>write!(f,"token: `-`"),
            Dot=>write!(f,"token: `.`"),
            Etc=>write!(f,"token: `...`"),
            Union=>write!(f,"token: `|`"),
            Add=>write!(f,"token: `+`"),
            Mul=>write!(f,"token: `*`"),
            Div=>write!(f,"token: `/`"),
            Mod=>write!(f,"token: `%`"),
            And=>write!(f,"token: `&`"),
            Not=>write!(f,"token: `!`"),
            SingleQuote=>write!(f,"token: `'`"),
            Backslash=>write!(f,"token: `\\`"),
            String(_)=>write!(f,"String"),
            Newline=>write!(f,"Newline"),
            Number(s)=>write!(f,"number: `{}`",s),
            Float(s)=>write!(f,"float: `{}`",s),
            DocComment(s)=>write!(f,"Doc comment"),
            Comment=>write!(f,"(internal compiler error) Comment"),
            Error=>write!(f,"(internal compiler error) Error"),
        }
    }
}
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Keyword {
    Function,
    Type,
    Interface,
    Pub,
    Mut,
    Impl,
    Import,
    For,
    While,
    Loop,
    Break,
    Return,
    Continue,
    Match,
    Enum,
    Module,
    This,
}
impl Display for Keyword {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use Keyword::*;
        match self {
            Function=>write!(f,"fn"),
            Type=>write!(f,"type"),
            Interface=>write!(f,"interface"),
            Pub=>write!(f,"pub"),
            Mut=>write!(f,"mut"),
            Impl=>write!(f,"impl"),
            Import=>write!(f,"import"),
            For=>write!(f,"for"),
            While=>write!(f,"while"),
            Loop=>write!(f,"loop"),
            Break=>write!(f,"break"),
            Return=>write!(f,"return"),
            Continue=>write!(f,"continue"),
            Match=>write!(f,"match"),
            Enum=>write!(f,"enum"),
            Module=>write!(f,"module"),
            This=>write!(f,"this"),
        }
    }
}


#[derive(Copy,Clone,Debug,PartialEq,Default)]
pub struct Location {
    pub line_start_index:usize,
    pub index:usize,
    pub line:usize,
    pub column:usize,
}
pub struct TokenIterator<'input> {
    input:&'input str,
    filename:&'input str,
    line:usize,
    line_start:usize,
    lexer:Lexer<'input,Token<'input>>,
}
impl<'input> TokenIterator<'input> {
    pub fn new(input:&'input str,filename:&'input str)->Self {
        TokenIterator {
            input,
            filename,
            line:0,
            line_start:0,
            lexer:Token::lexer(input),
        }
    }
}
impl<'input> Iterator for TokenIterator<'input> {
    type Item=Result<(Location,Token<'input>,Location),Error<'input,&'static str>>;
    fn next(&mut self)->Option<Self::Item> {
        let item=self.lexer.next()?;
        let span=self.lexer.span();
        println!("Token: {:?}",item);
        let start=Location {
            line_start_index:self.line_start,
            index:span.start,
            line:self.line,
            column:span.start-self.line_start,
        };
        match item {
            Token::Error=>Some(Err(Error{
                reason:"Invalid token",
                level:ErrorLevel::LexError,
                filename:self.filename,
                start,
                end:Location {
                    line_start_index:self.line_start,
                    index:span.end,
                    line:self.line,
                    column:span.end-self.line_start,
                },
            })),
            t=>{
                match &t {
                    Token::Newline=>{
                        self.line+=span.end-span.start;
                        self.line_start=span.end;
                    },
                    _=>{},
                }
                Some(Ok((
                    start,
                    t,
                    Location {
                        line_start_index:self.line_start,
                        index:span.end,
                        line:self.line,
                        column:span.end-self.line_start,
                    },
                )))
            },
        }
    }
}


fn line_doc_comment_fix<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
    return lex.slice().trim_end_matches('\n').trim_start_matches("///").trim();
}
fn block_doc_comment_fix<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
    return lex.slice().trim_end_matches("**/").trim_start_matches("/**").trim();
}
fn label_fix<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
    return lex.slice().trim_matches('^');
}
fn slice<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
    return lex.slice();
}
fn parse_raw_string<'input>(lexer:&mut Lexer<'input,Token<'input>>)->Option<&'input str> {
    const CHAR_SLICE:&[char]=&['r','"'];
    let closing=lexer.slice().trim_matches(CHAR_SLICE);
    lexer
        .remainder()
        .find(&closing)
        .map(|i|{
            lexer.bump(i);
            let ret=lexer.slice();
            lexer.bump(closing.len());
            let start=closing.len()+2;
            &ret[start..start+i-1]
        })
}
fn fix_string<'input>(lexer:&mut Lexer<'input,Token<'input>>)->&'input str {
    let s=lexer.slice();
    &s[1..s.len()-1]
}
