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
    #[token("return", |_|Keyword::Return)]
    #[token("continue", |_|Keyword::Continue)]
    #[token("match", |_|Keyword::Match)]
    #[token("enum", |_|Keyword::Enum)]
    #[token("module", |_|Keyword::Module)]
    #[token("this", |_|Keyword::This)]
    #[token("true", |_|Keyword::True)]
    #[token("false", |_|Keyword::False)]
    #[token("const", |_|Keyword::Const)]
    #[token("static", |_|Keyword::Static)]
    #[token("and", |_|Keyword::And)]
    #[token("or", |_|Keyword::Or)]
    #[token("in", |_|Keyword::In)]
    #[token("is", |_|Keyword::Is)]
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
    #[token("=>")]
    MatchSeparator,
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
    /// Disable block comments for now. We can't do syntax highlighting on them properly in
    /// tree-sitter and I don't know the proper regex for them.
    //#[regex("/\\*\\*[^**/]*\\*\\*/",block_doc_comment_fix)]
    DocComment(&'input str),
    #[regex("[ \t]+", logos::skip)]
    //#[regex("[ \t]+")]
    //Whitespace,
    #[regex("//[^\n]*",logos::skip)]
    //#[regex("/\\*[^*/]*\\*/",logos::skip)]
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
            MatchSeparator=>write!(f,"token: `=>`"),
            String(_)=>write!(f,"String"),
            Newline=>write!(f,"Newline"),
            Number(s)=>write!(f,"number: `{}`",s),
            Float(s)=>write!(f,"float: `{}`",s),
            DocComment(_)=>write!(f,"Doc comment"),
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
    Return,
    Continue,
    Match,
    Enum,
    Module,
    This,
    True,
    False,
    Const,
    Static,
    And,
    Or,
    In,
    Is,
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
            Return=>write!(f,"return"),
            Continue=>write!(f,"continue"),
            Match=>write!(f,"match"),
            Enum=>write!(f,"enum"),
            Module=>write!(f,"module"),
            This=>write!(f,"this"),
            True=>write!(f,"true"),
            False=>write!(f,"false"),
            Const=>write!(f,"const"),
            Static=>write!(f,"static"),
            And=>write!(f,"and"),
            Or=>write!(f,"or"),
            In=>write!(f,"in"),
            Is=>write!(f,"is"),
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
    //input:&'input str,
    skip_doc_comments:bool,
    filename:&'input str,
    line:usize,
    line_start:usize,
    lexer:Lexer<'input,Token<'input>>,
    next_token:Option<Token<'input>>,
}
impl<'input> TokenIterator<'input> {
    pub fn new(input:&'input str,filename:&'input str,skip_doc_comments:bool)->Self {
        TokenIterator {
            //input,
            skip_doc_comments,
            filename,
            line:0,
            line_start:0,
            lexer:Token::lexer(input),
            next_token:None,
        }
    }
}
impl<'input> Iterator for TokenIterator<'input> {
    type Item=Result<(Location,Token<'input>,Location),Error<'input,&'static str>>;
    fn next(&mut self)->Option<Self::Item> {
        if self.next_token.is_none() {
            self.next_token=self.lexer.next();
        }
        if self.next_token.is_none() {
            return None;
        }
        loop {
            let span=self.lexer.span();
            let start=Location {
                line_start_index:self.line_start,
                index:span.start,
                line:self.line,
                column:span.start-self.line_start,
            };
            let item=self.next_token.take()?;
            match item {
                Token::Newline=>{
                    self.next_token=self.lexer.next();
                    self.line+=span.end-span.start;
                    self.line_start=span.end;
                    println!("Token after newline: {:?}",self.next_token);
                    match &self.next_token {
                        Some(Token::Newline)=>continue,
                        Some(Token::DocComment(_))=>if self.skip_doc_comments{continue},
                        Some(Token::Comment)=>continue,
                        _=>{},
                    }
                    return Some(Ok((
                        start,
                        Token::Newline,
                        Location {
                            line_start_index:self.line_start,
                            index:span.end,
                            line:self.line,
                            column:span.end-self.line_start,
                        },
                    )));
                },
                Token::Error=>return Some(Err(Error{
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
                Token::DocComment(c)=>{
                    if self.skip_doc_comments {
                        self.next_token=self.lexer.next();
                        continue;
                    } else {
                        return Some(Ok((
                            start,
                            Token::DocComment(c),
                            Location {
                                line_start_index:self.line_start,
                                index:span.end,
                                line:self.line,
                                column:span.end-self.line_start,
                            },
                        )));
                    }
                },
                t=>return Some(Ok((
                    start,
                    t,
                    Location {
                        line_start_index:self.line_start,
                        index:span.end,
                        line:self.line,
                        column:span.end-self.line_start,
                    },
                ))),
            }
        }
    }
}


fn line_doc_comment_fix<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
    return lex.slice().trim_end_matches('\n').trim_start_matches("///").trim();
}
//fn block_doc_comment_fix<'input>(lex:&mut Lexer<'input,Token<'input>>)->&'input str {
//    return lex.slice().trim_end_matches("**/").trim_start_matches("/**").trim();
//}
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
