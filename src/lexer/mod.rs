
pub mod lexer {
    use std::{char, fs::File, io::{self, Read}, path::Path};

    #[derive(Debug)]
    enum Operators {
        Assign, // = 
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
        Binary([char; 2]), // e.g. << , >> , | , & , etc...
    }

    #[derive(Debug)]
    enum Keywords {
        Define,
        Mutable,
        Function,
        Type(String),
        Publicity,
        Null,
        Return,
    }   

    #[derive(Debug)]
    pub enum Token {
        Operator(Operators),
        Literal(String),
        Keyword(Keywords)
    }

    #[derive(Debug)]
    pub struct ParsingHead {
        curr_char: char,
        next_char: char,
        line: usize,
        column: usize,
    }

    #[derive(Debug)]
    struct Mistake {
        written: String,
        potential: String,
        other_potentials: Vec<String>,
    }

    #[derive(Debug)]
    pub enum ParsingError {
        InvalidOperator(Mistake),
        InvalidKeyword(Mistake),
        IoError(io::Error),
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

    impl std::error::Error for ParsingError {}

    impl ParsingHead {
        pub fn parse_source(path: &Path) -> Result<Vec<Token>, ParsingError> {
            let mut tokens: Vec<Token> = Vec::new();
            let mut file = File::open(path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            println!("{0:#?}", buf);
            todo!()
        }
    }
}
