mod lexer;
use lexer::lexer::ParsingHead;

use std::path::Path;


fn main() {
    let source = ParsingHead::parse_source(&Path::new("test.txt"));
    match source {
        Ok(_) => {},
        Err(error) => panic!("{0}", error)
    };
}