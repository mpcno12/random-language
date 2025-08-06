use std::path::Path;
use language_lexer;
fn main() {
    println!("Hello, world!");
    ParsingHead::parse_source(&Path::new("test.txt"));
}
