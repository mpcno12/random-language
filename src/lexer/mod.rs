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
        r"\A(?P<WHITESPACE>\s+)|",
        "(?P<COMMENT>//[^\n]*)|",
        r"(?P<DEFINE>let)|",
        r"(?P<MUTABLE>mut)|",
        r"(?P<FUNCTION>func)|",
        r"(?P<IF>if)|",
        r"(?P<ELSE>else)|",
        r"(?P<WHILE>while)|",
        r"(?P<PUBLICITY>public|private|interface)|",
        r"(?P<NULL>null)|",
        r"(?P<RETURN>return)|",
        r"(?P<NUMBER>\d+(\.\d+)?)|",
        r"(?P<IDENTIFIER>[A-Za-z_][A-Za-z0-9_]*)|",
        r#"(?P<STRING>\"(?:\\.|[^\"])*\")|"#,
        r"(?P<EQ>==)|",
        r"(?P<NE>!=)|",
        r"(?P<POWER>\*\*)|",
        r"(?P<ASSIGN>=)|",
        r"(?P<ADD>\+)|",
        r"(?P<SUBTRACT>-)|",
        r"(?P<ASTERISK>\*)|",
        r"(?P<DIVIDE>/)|",
        r"(?P<OPENPAREN>\()|",
        r"(?P<CLOSEPAREN>\))|",
        r"(?P<OPENBRACKET>{)|",
        r"(?P<CLOSEBRACKET>})|",
        r"(?P<OPENBRACE>\[)|",
        r"(?P<CLOSEBRACE>\])|",
        r"(?P<ENDLINE>;)"
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
        position: Position
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
            let mut token_kinds: Vec<TokenKind> = Vec::new();
            for caps in search.captures_iter(&source) {
                token_kinds.push(match () {
                    _ if matches!(caps.name("WHITESPACE"), m) => TokenKind::Ignore,
                    _ if matches!(caps.name("COMMENT"), m) => TokenKind::Ignore,
                    _ if matches!(caps.name("DEFINE"), m) => TokenKind::Keyword(Keywords::Define),
                    _ if matches!(caps.name("MUTABLE"), m) => TokenKind::Keyword(Keywords::Mutable),
                    _ if matches!(caps.name("FUNCTION"), m) => TokenKind::Keyword(Keywords::Function),
                    _ if matches!(caps.name("IF"), m) => TokenKind::Keyword(Keywords::If),
                    _ if matches!(caps.name("ELSE"), m) => TokenKind::Keyword(Keywords::Else),
                    _ if matches!(caps.name("WHILE"), m) => TokenKind::Keyword(Keywords::While),
                    _ if matches!(caps.name("PUBLICITY"), m) => TokenKind::Keyword(Keywords::Publicity),
                    _ if matches!(caps.name("NULL"), m) => TokenKind::Type,
                    _ if matches!(caps.name("RETURN"), m) => TokenKind::Operator(Operators::Return),
                    // Replace 0 with the actual number later
                    _ if matches!(caps.name("NUMBER"), m) => TokenKind::Number(0),
                    // Replace "" with actual Identifier Later
                    _ if matches!(caps.name("IDENTIFIER"), m) => TokenKind::Identifier("".to_string()),
                    _ if matches!(caps.name("STRING"), m) => TokenKind::String("".to_string()),
                    _ if matches!(caps.name("EQ"), m) => TokenKind::Operator(Operators::Eq),
                    _ if matches!(caps.name("NE"), m) => TokenKind::Operator(Operators::Ne),
                    _ if matches!(caps.name("POWER"), m) => TokenKind::Operator(Operators::Power),
                    _ if matches!(caps.name("ASSIGN"), m) => TokenKind::Operator(Operators::Assign),
                    _ if matches!(caps.name("ADD"), m) => TokenKind::Operator(Operators::Add),
                    _ if matches!(caps.name("SUBTRACT"), m) => TokenKind::Operator(Operators::Subtract),
                    _ if matches!(caps.name("ASTERISK"), m) => TokenKind::Operator(Operators::Asterisk),
                    _ if matches!(caps.name("DIVIDE"), m) => TokenKind::Operator(Operators::Divide),
                    _ if matches!(caps.name("OPENPAREN"), m) => TokenKind::Operator(Operators::OpenPara),
                    _ if matches!(caps.name("CLOSEPAREN"), m) => TokenKind::Operator(Operators::ClosePara),
                    _ if matches!(caps.name("OPENBRACE"), m) => TokenKind::Operator(Operators::OpenBrace),
                    _ if matches!(caps.name("CLOSEBRACE"), m) => TokenKind::Operator(Operators::CloseBrace),
                    _ if matches!(caps.name("OPENBRACKET"), m) => TokenKind::Operator(Operators::OpenBracket),
                    _ if matches!(caps.name("CLOSEBRACKET"), m) => TokenKind::Operator(Operators::CloseBracket),
                    _ if matches!(caps.name("ENDLINE"), m) => TokenKind::Operator(Operators::EndLine),
                    () => todo!()
                });
            };
            println!("{:#?}", token_kinds);
            todo!()
        }

        fn regex_parser() {}
    }
}
