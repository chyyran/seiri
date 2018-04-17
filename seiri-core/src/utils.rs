use std::io;
use bangs::Bang;
use bangs::lex_query;
pub fn wait_for_exit() {
    let stdin = io::stdin();
    println!("Type 'exit' to exit");

    let mut input = String::new();
    while let Ok(_) = stdin.read_line(&mut input) {
        if input.trim().eq_ignore_ascii_case("exit") {
            return;
        }
        if input.trim().starts_with("query") {
            let query_str : &str = input.trim().splitn(2, " ").last().unwrap();
            match lex_query(query_str) {
                Ok(query) => println!("{:?}", query),
                Err(err) => println!("{}", err)
            }
        }
        input.clear();
        continue;
    }
}
