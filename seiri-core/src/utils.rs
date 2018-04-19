use std::io;
use bangs::lex_query;
use bangs::parse_token_stream;
pub fn wait_for_exit() {
    let stdin = io::stdin();
    println!("Type 'exit' to exit");

    let mut input = String::new();
    while let Ok(_) = stdin.read_line(&mut input) {
        if input.trim().eq_ignore_ascii_case("exit") {
            return;
        }
        if input.trim().starts_with("query") {
            let query_str : &str = match input.trim().splitn(2, " ").nth(1) {
                Some(query_str) => query_str,
                None => ""
            };

            match lex_query(query_str) {
                Ok(query) => { 
                    println!("{:?}", query);
                    let parsed_bang = parse_token_stream(&mut query.iter());
                    match parsed_bang {
                        Ok(parsed) => {
                            println!("{:?}", parsed)
                        },
                        Err(err) => println!("{}", err)
                    }
                },
                Err(err) => println!("{}", err)
            }
        }
        input.clear();
        continue;
    }
}
