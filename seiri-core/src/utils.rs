use std::io;
use bangs::Bang;
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
            println!("{:?}", Bang::parse(query_str));
        }
        input.clear();
        continue;
    }
}
