extern crate quick_error;

use crate::bangs::{LexerMode, Token};
use std::result;
use std::path::PathBuf;
use quick_error::quick_error;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ConfigErrorType {
    IOError(String),
    Invalid,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnsupportedFile(file_name: PathBuf) {
            display(r#"The file "{:?}" is not supported or is not a music file"#, file_name)
        }
        FileNotFound(file_name: String) {
            display(r#"The file "{}" could not be found"#, file_name)
        }
        UnableToMove(file_name: String) {
            display(r#"The file {} could not be moved."#, file_name)
        }
        FileIOError(file_name:  PathBuf) {
            display(r#"The file {:?} could not be processed."#, file_name)
        }
        UnableToCreateDirectory(directory_name: String) {
            display(r#"The directory {} could not be created."#, directory_name)
        }
        UnsupportedOS {
            display("The operating system is unsupported.")
        }
        MissingRequiredTag(file_name: String, tag_name: &'static str) {
            display(r#"The track "{}" does not have the required tag {}"#, file_name, tag_name)
        }
        LexerUnexpectedCharacter(character: char, mode: LexerMode) {
            display(r#"Unexpected "{}" when lexing {:?}"#, character, mode)
        }
        LexerUnexpectedEscapeCharacter(mode: LexerMode) {
            display(r#"Unexpected escape '\\' when lexing {:?}"#, mode)
        }
        LexerUnexpectedEndOfInput {
            display(r#"Input ended before argument was fully parsed."#)
        }
        ParserUnexpectedToken(t: Token) {
            display(r#"Unexpected "{:?}" when parsing query"#, t)
        }
        ParserUnknownBang(b: String) {
            display(r#"Unknown bang !"{:?}" when parsing query"#, b)
        }
        ParserInvalidInput(input: String) {
            display(r#"Invalid input "{}" when parsing bang"#, input)
        }
        ConfigError(error: ConfigErrorType) {
            display(r#"Error "{:?}" when parsing configuration"#, error)
        }
    }
}
