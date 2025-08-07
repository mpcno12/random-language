mod lexer;
use lexer::lexer::Token;

use std::path::Path;

fn main() {
    let parsing_head = Token::new();
    let source = Token::parse_source(parsing_head, &Path::new("test.txt"));
    match source {
        Ok(_) => {}
        Err(error) => panic!("{0}", error),
    };
}
