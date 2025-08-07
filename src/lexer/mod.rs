#![allow(dead_code)] // To get the compiler to shut up.
pub mod lexer {
    use {
        regex::Regex,
        std::{
            fs::File, 
            io::{
                self, 
                Read
            },
            path::Path
        }
    };

    const REGEX_PATTERN: &'static str = concat!(
    r#"\A"#,
    r#"(?P<WHITESPACE>\s+)|"#,
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
    r#"(?i)"# 
);

    #[derive(Debug)]
    pub enum Operators {
        Assign, // = 
        Eq, // ==
        Ne, // !=
        Add, // +
        Subtract, // -
        Asterisk, // * , represents both pointers and multiplication operations
        Divide, // (divider) / 
        Power, // **
        OpenPara, // (
        ClosePara, // )
        Return, // ->
        OpenBracket, // {
        CloseBracket, // }
        OpenBrace, // [
        CloseBrace, // ]
        EndLine, // ;
        // TODO: Figure this out.
        // Binary([char; 2]), // e.g. << , >> , | , & , etc...
    }

    #[derive(Debug)]
    pub enum Keywords {
        Define, // let
        Mutable, // mut
        Function, // func
        // Todo: Figure this out
        // Type(String), // int, float, double
        If, // if
        Else, // else
        While, // while
        Publicity, // public, private, interface
        Null, // null, (Regex has issues parsing this; divides null & ptr) nullptr
        Return, // return
    }   

    #[derive(Debug, Default)]
    pub enum TokenKind {
        #[default] Ignore, // Start
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
        EmptyFile
    }

    impl std::fmt::Display for ParsingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::InvalidOperator(mistake) => {
                    writeln!(f, "Invalid Operator: expected {0}, found {1}", mistake.potential, mistake.written)?;
                    writeln!(f, "Similar Valid Operators: ")?;
                    let _ = mistake.other_potentials.iter().map(|item| writeln!(f, "    {0}", item));
                }
                Self::InvalidKeyword(mistake) => {
                    writeln!(f, "Invalid Keyword: expected {0}, found {1}", mistake.potential, mistake.written)?;
                    writeln!(f, "Similar Valid Operators: ")?;
                    let _ = mistake.other_potentials.iter().map(|item| writeln!(f, "    {0}", item));
                }
                Self::IoError(io_error) => {
                    writeln!(f, "{0}", io_error)?;
                },
                Self::RegexError(regex_error) => {
                    writeln!(f, "{0}", regex_error)?;
                },
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
            if source.is_empty() {return Err(ParsingError::EmptyFile)};
            let search = Regex::new(REGEX_PATTERN)?;
            let mut tokens: Vec<Token> = Vec::new();
            for caps in search.captures_iter(&source) {
                tokens.push(match () {
                    _ if matches!(caps.name("WHITESPACE"), _) => {
                            Self {
                                kind: TokenKind::Ignore,
                                text: String::default()
                            }
                        },
                    _ if matches!(caps.name("COMMENT"), _) => {
                        Self {
                            kind: TokenKind::Ignore,
                            text: caps.name("COMMENT").unwrap().as_str().to_string()
                        }
                    },
                    _ if matches!(caps.name("DEFINE"), _) => {Self{kind: TokenKind::Keyword(Keywords::Define), text: "let".to_string()}},
                    _ if matches!(caps.name("MUTABLE"), _) => {Self{kind: TokenKind::Keyword(Keywords::Mutable), text: "mut".to_string()}},
                    _ if matches!(caps.name("FUNCTION"), _) => {Self{kind: TokenKind::Keyword(Keywords::Function), text: "func".to_string()}},
                    _ if matches!(caps.name("IF"), _) => {Self{kind: TokenKind::Keyword(Keywords::If), text: "if".to_string()}}
                    _ if matches!(caps.name("ELSE"), _) => {Self{kind: TokenKind::Keyword(Keywords::Else), text: "else".to_string()}}
                    _ if matches!(caps.name("WHILE"), _) => {Self{kind: TokenKind::Keyword(Keywords::While), text: "while".to_string()}}
                    _ if matches!(caps.name("PUBLICITY"), _) => {Self{kind: TokenKind::Keyword(Keywords::Publicity), text: caps.name("PUBLICITY").unwrap().as_str().to_string()}}
                    _ if matches!(caps.name("NULL"), _) => {Self{kind: TokenKind::Type, text: "null".to_string()}}
                    _ if matches!(caps.name("RETURN"), _) => {Self{kind: TokenKind::Operator(Operators::Return), text: "return".to_string()}}

                    _ if matches!(caps.name("NUMBER"), _) => {Self{kind: TokenKind::Number(caps.name("NUMBER").unwrap().as_str().parse().unwrap()), text: caps.name("NUMBER").unwrap().as_str().to_string()}},
                    _ if matches!(caps.name("IDENTIFIER"), _) => {Self{kind: TokenKind::Identifier(caps.name("IDENTIFIER").unwrap().as_str().to_string()), text: caps.name("IDENTIFIER").unwrap().as_str().to_string()}}
                    _ if matches!(caps.name("STRING"), _) => {Self{kind: TokenKind::String(caps.name("STRING").unwrap().as_str().to_string()), text: caps.name("STRING").unwrap().as_str().to_string()}}
                    _ if matches!(caps.name("EQ"), _) => {Self{kind: TokenKind::Operator(Operators::Eq), text: "==".to_string()}}
                    _ if matches!(caps.name("NE"), _) => {Self{kind: TokenKind::Operator(Operators::Ne), text: "!=".to_string()}}
                    _ if matches!(caps.name("POWER"), _) => {Self{kind: TokenKind::Operator(Operators::Power), text: "**".to_string()}}
                    _ if matches!(caps.name("ASSIGN"), _) => {Self{kind: TokenKind::Operator(Operators::Assign), text: "=".to_string()}}
                    _ if matches!(caps.name("ADD"), _) => {Self{kind: TokenKind::Operator(Operators::Add), text: "+".to_string()}}
                    _ if matches!(caps.name("SUBTRACT"), _) => {Self{kind: TokenKind::Operator(Operators::Subtract), text: "-".to_string()}}
                    _ if matches!(caps.name("ASTERISK"), _) => {Self{kind: TokenKind::Operator(Operators::Asterisk), text: "*".to_string()}}
                    _ if matches!(caps.name("DIVIDE"), _) => {Self{kind: TokenKind::Operator(Operators::Divide), text: r"/".to_string()}}
                    _ if matches!(caps.name("OPENPAREN"), _) => {Self{kind: TokenKind::Operator(Operators::OpenPara), text: "(".to_string()}}
                    _ if matches!(caps.name("CLOSEPAREN"), _) => {Self{kind: TokenKind::Operator(Operators::ClosePara), text: ")".to_string()}}
                    _ if matches!(caps.name("OPENBRACE"), _) => {Self{kind: TokenKind::Operator(Operators::OpenBrace), text: "[".to_string()}}
                    _ if matches!(caps.name("CLOSEBRACE"), _) => {Self{kind: TokenKind::Operator(Operators::CloseBrace), text: "]".to_string()}}
                    _ if matches!(caps.name("OPENBRACKET"), _) => {Self{kind: TokenKind::Operator(Operators::OpenBracket), text: "{".to_string()}}
                    _ if matches!(caps.name("CLOSEBRACKET"), _) => {Self{kind: TokenKind::Operator(Operators::CloseBracket), text: "}".to_string()}}
                    _ if matches!(caps.name("ENDLINE"), _) => {Self{kind: TokenKind::Operator(Operators::EndLine), text: ";".to_string()}}
                    () => unimplemented!()
                });
            };
            println!("{:#?}", tokens);
            return Ok(tokens);
        }

        fn regex_parser() {}
    }
}
