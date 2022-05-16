use crate::error::*;


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Keyword {
    Public,
    Mutable,
    Constraint,
    Init,
    Import,
    Return,
}
impl TryFrom<&str> for Keyword {
    type Error=();
    fn try_from(word:&str)->Result<Self,()> {
        use Keyword::*;
        match word {
            "constraint"=>Ok(Constraint),
            "import"=>Ok(Import),
            "public"=>Ok(Public),
            "mutable"=>Ok(Mutable),
            "return"=>Ok(Return),
            "init"=>Ok(Init),
            _=>Err(()),
        }
    }
}
#[derive(Clone,Debug,PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    Integer(&'a str),
    Float(&'a str),
    BlockComment(&'a str),
    LineComment(&'a str),
    Keyword(Keyword),
    Unknown(char),
    StringToken(String),
    Space(usize),       // ' '
    Tab(usize),         // \t
    Newline(usize),     // \n
    CurlyBracketStart,  // {
    CurlyBracketEnd,    // }
    SquareBracketStart, // [
    SquareBracketEnd,   // ]
    ParenthesisStart,   // (
    ParenthesisEnd,     // )
    AngleBracketStart,  // <
    AngleBracketEnd,    // >
    Ref,                // &
    RefMut,             // *
    Add,                // +
    Union,              // |
    Colon,              // :
    DeclareVar,         // :=
    PathDelimeter,      // ::
    Assign,             // =
    Dot,                // .
    At,                 // @
    Hash,               // #
    Comma,              // ,
    SingleQuote,        // '
    Semicolon,          // ;
    Tilde,              // ~
}
#[derive(Clone,Debug,PartialEq)]
enum TokenState {
    SOI,
    WordState {
        start:usize,
        end:usize,
    },
    NumberState {
        start:usize,
        end:usize,
        dot:bool,
    },
    LineCommentState {
        start:usize,
        end:usize,
    },
    BlockCommentState {
        start:usize,
        end:usize,
        level:usize,
    },
    SpaceState {
        count:usize,
    },
    TabState {
        count:usize,
    },
    NewlineState {
        count:usize,
    },
    StringState {
        start:usize,
        data:String,
    },
    None,
}
#[derive(Clone,Debug,PartialEq)]
pub struct TokenList<'a>(pub Vec<Token<'a>>);
impl<'a> TokenList<'a> {
    #[inline]
    pub fn new(s:&'a str)->Result<Self,Error> {
        Self::try_from(s)
    }
    #[cfg(debug_assertions)]
    pub fn recreate_source(&self)->String {
        let mut source=String::new();
        for token in self.0.iter() {
            use Token::*;
            match token {
                Tilde=>source.push('~'),
                Comma=>source.push(','),
                SingleQuote=>source.push('\''),
                Semicolon=>source.push(';'),
                Dot=>source.push('.'),
                At=>source.push('@'),
                Hash=>source.push('#'),
                PathDelimeter=>source.push_str("::"),
                Colon=>source.push(':'),
                DeclareVar=>source.push_str(":="),
                Assign=>source.push('='),
                Ref=>source.push('&'),
                RefMut=>source.push('*'),
                Add=>source.push('+'),
                Union=>source.push('|'),
                ParenthesisStart=>source.push('('),
                ParenthesisEnd=>source.push(')'),
                CurlyBracketStart=>source.push('{'),
                CurlyBracketEnd=>source.push('}'),
                SquareBracketStart=>source.push('['),
                SquareBracketEnd=>source.push(']'),
                AngleBracketStart=>source.push('<'),
                AngleBracketEnd=>source.push('>'),
                Word(s)|Integer(s)|Float(s)=>source.push_str(s),
                BlockComment(s)=>{
                    source.push_str("/*\n");
                    source.push_str(s);
                    source.push_str("\n*/");
                },
                LineComment(s)=>{
                    source.push_str("// ");
                    source.push_str(s);
                    source.push('\n');
                },
                Unknown(c)=>source.push(*c),
                StringToken(s)=>{
                    source.push('"');
                    let escaped=s.escape_default().to_string();
                    source.push_str(&escaped);
                    source.push('"');
                },
                Space(count)=>(0..*count).for_each(|_|source.push(' ')),
                Newline(count)=>(0..*count).for_each(|_|source.push('\n')),
                Tab(count)=>(0..*count).for_each(|_|source.push('\t')),
                Keyword(k)=>{
                    use self::Keyword::*;
                    match k {
                        Constraint=>source.push_str("constraint"),
                        Public=>source.push_str("public"),
                        Mutable=>source.push_str("mutable"),
                        Init=>source.push_str("init"),
                        Import=>source.push_str("import"),
                        Return=>source.push_str("return"),
                    }
                },
            }
        }
        return source;
    }
}
impl<'a> TryFrom<&'a str> for TokenList<'a> {
    type Error=Error;
    fn try_from(s:&'a str)->Result<Self,Self::Error> {
        use TokenState::*;
        use Token::*;
        let mut chars=s.chars().enumerate().peekable();
        let mut tokens=Vec::new();
        let mut state:TokenState=SOI;
        let mut line=1;
        let mut column=1;
        while let Some((i,c))=chars.next() {
            if state==None||state==SOI {
                match c {
                    '\r'=>{},
                    ' '=>state=SpaceState{count:0},
                    '\t'=>state=TabState{count:0},
                    '\n'=>state=NewlineState{count:0},
                    '('=>tokens.push(ParenthesisStart),
                    ')'=>tokens.push(ParenthesisEnd),
                    '['=>tokens.push(SquareBracketStart),
                    ']'=>tokens.push(SquareBracketEnd),
                    '{'=>tokens.push(CurlyBracketStart),
                    '}'=>tokens.push(CurlyBracketEnd),
                    '<'=>tokens.push(AngleBracketStart),
                    '>'=>tokens.push(AngleBracketEnd),
                    '='=>tokens.push(Assign),
                    '@'=>tokens.push(At),
                    '+'=>tokens.push(Add),
                    '|'=>tokens.push(Union),
                    '.'=>tokens.push(Dot),
                    '&'=>tokens.push(Ref),
                    '*'=>tokens.push(RefMut),
                    '#'=>tokens.push(Hash),
                    '\''=>tokens.push(SingleQuote),
                    '"'=>state=StringState{start:i,data:String::new()},
                    ';'=>tokens.push(Semicolon),
                    '~'=>tokens.push(Tilde),
                    ','=>tokens.push(Comma),
                    '0'..='9'=>state=NumberState{start:i,end:i,dot:false},
                    'a'..='z'|'A'..='Z'|'_'=>state=WordState{start:i,end:i},
                    '/'=>{
                        if let Some((_,'/'))=chars.peek() {
                            chars.next();
                            state=LineCommentState{start:i,end:i};
                        } else if let Some((_,'*'))=chars.peek() {
                            chars.next();
                            state=BlockCommentState{start:i,end:i,level:0};
                        }
                    },
                    ':'=>{
                        let peek=chars.peek();
                        if let Some((_,'='))=peek {
                            chars.next();
                            tokens.push(DeclareVar);
                        } else if let Some((_,':'))=peek {
                            chars.next();
                            tokens.push(PathDelimeter);
                        } else {
                            tokens.push(Colon);
                        }
                    },
                    _=>tokens.push(Unknown(c)),
                }
            }
            match &mut state {
                WordState{end,start}=>{
                    let invalid_chars=[
                        // Names can be of any length and contain any unicode characters except `.`, `(`, ` ` (space),
                        // `\n` (newline), `=`, `:`, `~`, `+`, `^`, `-`, `|`, `,`, `)`, `<`, `>`, `[`, `]`.
                        ' ',
                        '\t',
                        '\n',
                        '\r',
                        '.',
                        '"',
                        '=',
                        '~',
                        '+',
                        '-',
                        ',',
                        ':',
                        '|',
                        '^',
                        '(',')',
                        '[',']',
                        '{','}',
                        '<','>',
                    ];
                    *end=i;
                    // if the next char is not a valid word char
                    if chars.peek().map(|(_,c)|invalid_chars.contains(c)).unwrap_or(true) {
                        let s=&s[*start..=*end];
                        if let Ok(kw)=s.try_into() {
                            tokens.push(Keyword(kw));
                        } else {
                            tokens.push(Word(s));
                        }
                        state=None;
                    }
                },
                LineCommentState{end,start}=>{
                    *end=i;
                    match c {
                        '\n'=>{
                            tokens.push(LineComment(s[*start+2..i].trim()));
                            state=None;
                        },
                        _=>{},
                    }
                },
                BlockCommentState{end,level,start}=>{
                    *end=i;
                    match c {
                        '*'=>{
                            if let Some((_,'/'))=chars.peek() {
                                chars.next();
                                if *level==0 {
                                    tokens.push(BlockComment(s[*start+2..i].trim()));
                                    state=None;
                                } else {
                                    *level-=1;
                                }
                            }
                        },
                        '/'=>{
                            if let Some((_,'*'))=chars.peek() {
                                chars.next();
                                *level+=1;
                            }
                        },
                        _=>{},
                    }
                },
                NumberState{end,start,dot}=>{
                    *end=i;
                    match c {
                        '.'=>{
                            if *dot {
                                return Err(Error{line,column,error:ErrorType::MultipleDecimal});
                            } else {
                                *dot=true;
                            }
                        },
                        _=>{
                            let next_is_valid_number_char=chars.peek().map(|(_,c)|c.is_ascii_digit()||*c=='.').unwrap_or(true);
                            if !next_is_valid_number_char {
                                if *dot {
                                    tokens.push(Float(&s[*start..=*end]));
                                } else {
                                    tokens.push(Integer(&s[*start..=*end]));
                                }
                                state=None;
                            }
                        },
                    }
                },
                NewlineState{count}=>{
                    *count+=1;
                    if let Some((_,c))=chars.peek() {
                        if *c!='\n' {
                            tokens.push(Newline(*count));
                            state=None;
                        }
                    }
                },
                SpaceState{count}=>{
                    *count+=1;
                    if let Some((_,c))=chars.peek() {
                        if *c!=' ' {
                            tokens.push(Space(*count));
                            state=None;
                        }
                    }
                },
                TabState{count}=>{
                    *count+=1;
                    if let Some((_,c))=chars.peek() {
                        if *c!='\t' {
                            tokens.push(Tab(*count));
                            state=None;
                        }
                    }
                },
                StringState{start,data}=>{
                    match c {
                        '\\'=>{
                            if let Some((_,c))=chars.next() {
                                column+=1;
                                match c {
                                    'n'=>data.push('\n'),
                                    't'=>data.push('\t'),
                                    'r'=>data.push('\r'),
                                    'u'=>{
                                        // unicode escape: `\u{HEX_NUM}`
                                        if let Some((_,c))=chars.next() {
                                            column+=1;
                                            if c!='{' {
                                                return Err(Error{line,column,error:ErrorType::ExpectedUnicodeEscapeStart(c)});
                                            }
                                            let mut escape_chars=String::new();
                                            loop {
                                                if let Some((_,c))=chars.next() {
                                                    column+=1;
                                                    match c {
                                                        '}'=>break,
                                                        'a'..='f'|'A'..='F'|'0'..='9'=>{
                                                            if escape_chars.len()==6 {
                                                                return Err(Error{line,column,error:ErrorType::ExpectedUnicodeEscapeEnd(c)});
                                                            }
                                                            escape_chars.push(c);
                                                        },
                                                        _=>return Err(Error{line,column,error:ErrorType::ExpectedUnicodeEscape(c)}),
                                                    }
                                                } else {
                                                    return Err(Error{line,column,error:ErrorType::UnexpectedEOFString});
                                                }
                                            }
                                            let num=u32::from_str_radix(&escape_chars,16).expect("Internal compiler error: could not parse the hex number that we just verified");
                                            if let Some(c)=char::from_u32(num) {
                                                data.push(c);
                                            } else {
                                                return Err(Error{line,column,error:ErrorType::UnicodeEscapeOutOfRange(num)});
                                            }
                                        }
                                    },
                                    'x'=>{
                                        // ascii escape: `\x__` where the `__` is a hex number between
                                        // 00 an 7f, inclusive
                                        let mut s=String::new();
                                        for _ in 0..2 {
                                            if let Some((_,c))=chars.next() {
                                                column+=1;
                                                match c {
                                                    '0'..='9'|'a'..='f'|'A'..='F'=>{
                                                        s.push(c);
                                                    },
                                                    _=>return Err(Error{line,column,error:ErrorType::ExpectedAsciiEscape(c)}),
                                                }
                                            } else {
                                                return Err(Error{line,column,error:ErrorType::UnexpectedEOFString});
                                            }
                                        }
                                        let num=u32::from_str_radix(&s,16).expect("Internal compiler error: could not parse the hex number that we just verified");
                                        if num<=0x7f {
                                            data.push(char::from_u32(num).unwrap());
                                        } else {
                                            return Err(Error{line,column,error:ErrorType::AsciiEscapeOutOfRange(num)});
                                        }
                                    },
                                    '0'=>data.push('\0'),
                                    '\\'=>data.push('\\'),
                                    _=>{
                                        return Err(Error{line,column,error:ErrorType::UnknownEscape(c)});
                                    },
                                }
                            } else {
                                return Err(Error{line,column,error:ErrorType::UnexpectedEOFString});
                            }
                        },
                        '"'=>{
                            if i!=*start {
                                tokens.push(StringToken(data.clone()));
                                state=None;
                            }
                        },
                        _=>data.push(c),
                    }
                },
                SOI|None=>{},
            }
            if c=='\n' {
                column=1;
                line+=1;
            } else {
                column+=1;
            }
        }
        match state {
            NewlineState{count}=>tokens.push(Newline(count)),
            TabState{count}=>tokens.push(Tab(count)),
            SpaceState{count}=>tokens.push(Space(count)),
            NumberState{start,end,dot}=>{
                if dot {
                    tokens.push(Float(&s[start..=end]));
                } else {
                    tokens.push(Integer(&s[start..=end]));
                }
            },
            WordState{start,end}=>{
                let s=&s[start..=end];
                if let Ok(kw)=s.try_into() {
                    tokens.push(Keyword(kw));
                } else {
                    tokens.push(Word(s));
                }
            },
            LineCommentState{start,end}=>tokens.push(LineComment(s[start..=end].trim())),
            BlockCommentState{..}=>return Err(Error{line,column,error:ErrorType::UnclosedBlockComment}),
            StringState{..}=>return Err(Error{line,column,error:ErrorType::UnexpectedEOFString}),
            SOI|None=>{},
        }
        return Ok(TokenList(tokens));
    }
}
