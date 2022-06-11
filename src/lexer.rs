use logos::{
    Logos,
    Lexer,
};


#[derive(Logos,Debug,PartialEq)]
pub enum Token<'a> {
    #[token("fn", |_|Keyword::Function)]
    #[token("type", |_|Keyword::Type)]
    #[token("interface", |_|Keyword::Interface)]
    #[token("pub", |_|Keyword::Pub)]
    #[token("mut", |_|Keyword::Mut)]
    #[token("impl", |_|Keyword::Impl)]
    #[token("import", |_|Keyword::Import)]
    #[token("for", |_|Keyword::For)]
    #[token("while", |_|Keyword::While)]
    #[token("loop", |_|Keyword::Loop)]
    #[token("break", |_|Keyword::Break)]
    #[token("return", |_|Keyword::Return)]
    #[token("continue", |_|Keyword::Continue)]
    #[token("match", |_|Keyword::Match)]
    Keyword(Keyword),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex|lex.slice())]
    Word(&'a str),
    #[token(",")]
    Comma,
    #[regex("'",parse_char)]
    Char(char),
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
    #[token("=")]
    Assign,
    #[token(".")]
    Dot,
    #[token("...")]
    Etc,
    #[regex("r#*\"",parse_raw_string)]
    #[regex("\"[^\"]*\"",fix_string)]
    String(&'a str),
    #[regex("\n+")]
    Newline,
    #[regex("-?[0-9][0-9_]*", |lex|lex.slice())]
    Number(&'a str),
    #[regex("-?[0-9]+\\.[0-9]+", |lex|lex.slice())]
    #[regex("-?\\.[0-9]+", |lex|lex.slice())]
    #[regex("-?[0-9]+\\.", |lex|lex.slice())]
    Float(&'a str),
    #[regex("[ \t]+")]
    Whitespace,
    #[regex("//.*\n",logos::skip)]
    #[regex("/\\*.*\\*/\n",logos::skip)]
    #[error]
    Error,
}
#[derive(Debug,PartialEq)]
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
}


fn parse_raw_string<'a>(lexer:&mut Lexer<'a,Token<'a>>)->Option<&'a str> {
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
fn fix_string<'a>(lexer:&mut Lexer<'a,Token<'a>>)->&'a str {
    let s=lexer.slice();
    &s[1..s.len()-1]
}
fn parse_char<'a>(lexer:&mut Lexer<'a,Token<'a>>)->Option<char> {
    let inner=lexer
        .remainder()
        .find('\'')
        .map(|i|{
            lexer.bump(i);
            let ret=lexer.slice();
            lexer.bump(1);
            &ret[1..i+1]
        })?.chars().collect::<Vec<_>>();
    match inner[0] {
        '\\'=>{
            match inner[1] {
                'n'=>return Some('\n'),
                't'=>return Some('\t'),
                'r'=>return Some('\r'),
                '0'=>return Some('\0'),
                'x'=>{
                    if inner.len()==4 {
                        if inner[2].is_ascii_hexdigit()&&inner[3].is_ascii_hexdigit() {
                            let s=format!("{}{}",inner[2],inner[3]);
                            return Some(u8::from_str_radix(&s,16).unwrap() as char);
                        }
                    }
                    return None;
                },
                '\\'=>return Some('\\'),
                _=>return None,
            }
        },
        c=>return Some(c),
    }
}
