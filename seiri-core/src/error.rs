extern crate quick_error;

use std::result;

pub type Result<T> = result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnsupportedFile(file_name: String) {
            description("File is not supported or is not a music file.")
            display(r#"The file "{}" is not supported or is not a music file"#, file_name)
        }
        FileNotFound(file_name: String) {
            description("The file could not be found")
            display(r#"The file "{}" could not be found"#, file_name)
        }
        UnsupportedOS {
            description("The operating system is unuspported.")
            display("The operating system is unsupported.")
        }
        HelperNotFound {
            description("Katatsuki tag helper was not found in tools.")
            display("Katatsuki tag helper was not found in tools.")
        }
        MissingRequiredTag(file_name: String, tag_name: &'static str) {
            description("Track does not contain the required tag.")
            display(r#"The track "{}" does not have the required tag {}"#, file_name, tag_name)
        }
    }
}
