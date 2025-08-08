#![allow(dead_code)] // To get the compiler to shut up.
pub mod lexer {
    use {
        regex::Regex,
        std::{
            fs::File,
            io::{self, Read},
            path::Path,
        },
    };

    const REGEX_PATTERN: &'static str = concat!(
        r#"(?mi)",
        r#"(?P<COMMENT>//[^\n]*)|"#,
        r#"(?P<DEFINE>let)|"#,
        r#"(?P<MUTABLE>mut)|"#,
        r#"(?P<FUNCTION>func)|"#,
        r#"(?P<IF>if)|"#,
        r#"(?P<ELSE>else)|"#,
        r#"(?P<WHILE>while)|"#,
        r#"(?P<PUBLICITY>public|private|interface)|"#,
        r#"(?P<NULL>nullptr|null)|"#,
        r#"(?P<RETURN>return)|"#,
        r#"(?P<NUMBER>\d+(\.\d+)?)|"#,
        r#"(?P<STRING>"(?:\\.|[^"\\])*")|"#,
        r#"(?P<IDENTIFIER>[A-Za-z_][A-Za-z0-9_]*)|"#,
        r#"(?P<EQ>==)|"#,
        r#"(?P<NE>!=)|"#,
        r#"(?P<POWER>\*\*)|"#,
        r#"(?P<ASSIGN>=)|"#,
        r#"(?P<ADD>\+)|"#,
        r#"(?P<SUBTRACT>-)|"#,
        r#"(?P<ASTERISK>\*)|"#,
        r#"(?P<DIVIDE>/)|"#,
        r#"(?P<OPENPAREN>\()|"#,
        r#"(?P<CLOSEPAREN>\))|"#,
        r#"(?P<OPENBRACKET>\{)|"#,
        r#"(?P<CLOSEBRACKET>\})|"#,
        r#"(?P<OPENBRACE>\[)|"#,
        r#"(?P<CLOSEBRACE>\])|"#,
        r#"(?P<ENDLINE>;)"#,
    );

    #[derive(Debug)]
    pub enum Operators {
        Assign,       // =
        Eq,           // ==
        Ne,           // !=
        Add,          // +
        Subtract,     // -
        Asterisk,     // * , represents both pointers and multiplication operations
        Divide,       // (divider) /
        Power,        // **
        OpenParen,     // (
        CloseParen,    // )
        Return,       // ->
        OpenBracket,  // {
        CloseBracket, // }
        OpenBrace,    // [
        CloseBrace,   // ]
        EndLine,      // ;
                      // TODO: Figure this out.
                      // Binary([char; 2]), // e.g. << , >> , | , & , etc...
    }

    #[derive(Debug)]
    pub enum Keywords {
        Define,   // let
        Mutable,  // mut
        Function, // func
        // Todo: Figure this out
        // Type(String), // int, float, double
        If,        // if
        Else,      // else
        While,     // while
        Publicity, // public, private, interface
        Null,      // null, (Regex has issues parsing this; divides null & ptr) nullptr
        Return,    // return
    }

    #[derive(Debug, Default)]
    pub enum TokenKind {
        #[default]
        Ignore, // Start
        Unknown, // I Dont know????
        Operator(Operators),
        // Todo: Figure this out
        // Only reason its avaliable is because its important
        Type,
        Number(i64),
        Identifier(String),
        String(String),
        Keyword(Keywords),
    }

    #[derive(Debug, Default)]
    pub struct Position {
        line: usize,
        column: usize,
    }

    #[derive(Debug, Default)]
    pub struct Token {
        kind: TokenKind,
        text: String,
    }

    #[derive(Debug)]
    pub struct Mistake {
        written: String,
        potential: String,
        other_potentials: Vec<String>,
    }

    #[derive(Debug)]
    pub enum ParsingError {
        InvalidOperator(Mistake),
        InvalidKeyword(Mistake),
        IoError(io::Error),
        RegexError(regex::Error),
        EmptyFile,
    }

    impl std::fmt::Display for ParsingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::InvalidOperator(mistake) => {
                    writeln!(
                        f,
                        "Invalid Operator: expected {0}, found {1}",
                        mistake.potential, mistake.written
                    )?;
                    writeln!(f, "Similar Valid Operators: ")?;
                    let _ = mistake
                        .other_potentials
                        .iter()
                        .map(|item| writeln!(f, "    {0}", item));
                }
                Self::InvalidKeyword(mistake) => {
                    writeln!(
                        f,
                        "Invalid Keyword: expected {0}, found {1}",
                        mistake.potential, mistake.written
                    )?;
                    writeln!(f, "Similar Valid Operators: ")?;
                    let _ = mistake
                        .other_potentials
                        .iter()
                        .map(|item| writeln!(f, "    {0}", item));
                }
                Self::IoError(io_error) => {
                    writeln!(f, "{0}", io_error)?;
                }
                Self::RegexError(regex_error) => {
                    writeln!(f, "{0}", regex_error)?;
                }
                Self::EmptyFile => {
                    writeln!(f, "The file is currently empty, please add code")?;
                }
            };
            return Ok(());
        }
    }
    impl From<io::Error> for ParsingError {
        fn from(value: io::Error) -> Self {
            return Self::IoError(value);
        }
    }
    impl From<regex::Error> for ParsingError {
        fn from(value: regex::Error) -> Self {
            Self::RegexError(value)
        }
    }
    impl std::error::Error for ParsingError {}

    impl Token {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn parse_source(self, path: &Path) -> Result<Vec<Token>, ParsingError> {
            let mut file = File::open(path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            let source = String::from_utf8(buf).expect("Invalid Source Code; Not UTF-8 Valid");
            if source.is_empty() {
                return Err(ParsingError::EmptyFile);
            };
            let search = Regex::new(REGEX_PATTERN)?;
            let mut tokens: Vec<Token> = Vec::new();
            for caps in search.captures_iter(&source) {
                tokens.push(match () {
                    _ if matches!(caps.name("COMMENT"), Some(_)) => Self {
                        kind: TokenKind::Ignore,
                        text: caps.name("COMMENT").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("WHITESPACE"), Some(_)) => Self {
                        kind: TokenKind::Ignore,
                        text: String::default(),
                    },
                    _ if matches!(caps.name("DEFINE"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Define),
                        text: "let".to_string(),
                    },
                    _ if matches!(caps.name("MUTABLE"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Mutable),
                        text: "mut".to_string(),
                    },
                    _ if matches!(caps.name("FUNCTION"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Function),
                        text: "func".to_string(),
                    },
                    _ if matches!(caps.name("IF"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::If),
                        text: "if".to_string(),
                    },
                    _ if matches!(caps.name("ELSE"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Else),
                        text: "else".to_string(),
                    },
                    _ if matches!(caps.name("WHILE"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::While),
                        text: "while".to_string(),
                    },
                    _ if matches!(caps.name("PUBLICITY"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Publicity),
                        text: caps.name("PUBLICITY").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("NULL"), Some(_)) => Self {
                        kind: TokenKind::Keyword(Keywords::Null),
                        text: caps.name("NULL").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("RETURN"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Return),
                        text: "return".to_string(),
                    },

                    _ if matches!(caps.name("NUMBER"), Some(_)) => Self {
                        kind: TokenKind::Number(
                            caps.name("NUMBER").unwrap().as_str().parse().unwrap(),
                        ),
                        text: caps.name("NUMBER").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("IDENTIFIER"), Some(_)) => Self {
                        kind: TokenKind::Identifier(
                            caps.name("IDENTIFIER").unwrap().as_str().to_string(),
                        ),
                        text: caps.name("IDENTIFIER").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("STRING"), Some(_)) => Self {
                        kind: TokenKind::String(caps.name("STRING").unwrap().as_str().to_string()),
                        text: caps.name("STRING").unwrap().as_str().to_string(),
                    },
                    _ if matches!(caps.name("EQ"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Eq),
                        text: "==".to_string(),
                    },
                    _ if matches!(caps.name("NE"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Ne),
                        text: "!=".to_string(),
                    },
                    _ if matches!(caps.name("POWER"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Power),
                        text: "**".to_string(),
                    },
                    _ if matches!(caps.name("ASSIGN"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Assign),
                        text: "=".to_string(),
                    },
                    _ if matches!(caps.name("ADD"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Add),
                        text: "+".to_string(),
                    },
                    _ if matches!(caps.name("SUBTRACT"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Subtract),
                        text: "-".to_string(),
                    },
                    _ if matches!(caps.name("ASTERISK"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Asterisk),
                        text: "*".to_string(),
                    },
                    _ if matches!(caps.name("DIVIDE"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::Divide),
                        text: r"/".to_string(),
                    },
                    _ if matches!(caps.name("OPENPAREN"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::OpenParen),
                        text: "(".to_string(),
                    },
                    _ if matches!(caps.name("CLOSEPAREN"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::CloseParen),
                        text: ")".to_string(),
                    },
                    _ if matches!(caps.name("OPENBRACE"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::OpenBrace),
                        text: "[".to_string(),
                    },
                    _ if matches!(caps.name("CLOSEBRACE"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::CloseBrace),
                        text: "]".to_string(),
                    },
                    _ if matches!(caps.name("OPENBRACKET"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::OpenBracket),
                        text: "{".to_string(),
                    },
                    _ if matches!(caps.name("CLOSEBRACKET"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::CloseBracket),
                        text: "}".to_string(),
                    },
                    _ if matches!(caps.name("ENDLINE"), Some(_)) => Self {
                        kind: TokenKind::Operator(Operators::EndLine),
                        text: ";".to_string(),
                    },
                    
                    () => Self {
                        kind: TokenKind::Unknown,
                        text: "wtf".to_string()
                    },
                });
            }
            println!("{:#?}", tokens);
            return Ok(tokens);
        }

        fn regex_parser() {}
    }
}
