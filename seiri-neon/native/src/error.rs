extern crate quick_error;

use bangs::LexerMode;
use bangs::Token;
use std::result;

pub type Result<T> = result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        LexerUnexpectedCharacter(character: char, mode: LexerMode) {
            description("Unexpected character when lexing query string.")
            display(r#"Unexpected "{}" when lexing {:?}"#, character, mode)
        }
        LexerUnexpectedEscapeCharacter(mode: LexerMode) {
            description("Escape character occurred at the end of the input.")
            display(r#"Unexpected escape '\\' when lexing {:?}"#, mode)
        }
        LexerUnexpectedEndOfInput {
            description("Unexpected end of input")
            display(r#"Input ended before argument was fully parsed."#)
        }
        ParserUnexpectedToken(t: Token) {
            description("Unexpected token during parsing of query token stream.")
            display(r#"Unexpected "{:?}" when parsing query"#, t)
        }
        ParserUnknownBang(b: String) {
            description("Unknown bang during parsing of query token stream.")
            display(r#"Unknown bang !"{:?}" when parsing query"#, b)
        }
        ParserInvalidInput(input: String) {
            description("Invalid input when parsing bang argument")
            display(r#"Invalid input "{}" when parsing bang"#, input)
        }
    }
}
