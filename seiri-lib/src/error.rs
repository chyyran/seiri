extern crate quick_error;

use bangs::LexerMode;
use bangs::Token;
use std::result;
use std::path::PathBuf;

pub type Result<T> = result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnsupportedFile(file_name: PathBuf) {
            description("File is not supported or is not a music file.")
            display(r#"The file "{:?}" is not supported or is not a music file"#, file_name)
        }
        FileNotFound(file_name: String) {
            description("The file could not be found")
            display(r#"The file "{}" could not be found"#, file_name)
        }
        UnableToMove(file_name: String) {
            description("The file could not be moved")
            display(r#"The file {} could not be moved."#, file_name)
        }
        UnableToCreateDirectory(directory_name: String) {
            description("The directory could not be created")
            display(r#"The directory {} could not be created."#, directory_name)
        }
        UnsupportedOS {
            description("The operating system is unuspported.")
            display("The operating system is unsupported.")
        }
        MissingRequiredTag(file_name: String, tag_name: &'static str) {
            description("Track does not contain the required tag.")
            display(r#"The track "{}" does not have the required tag {}"#, file_name, tag_name)
        }
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
