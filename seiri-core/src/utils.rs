use std::io;

pub fn wait_for_exit() {
    let stdin = io::stdin();
    println!("Type 'exit' to exit");

    let mut input = String::new();
    while let Ok(_) = stdin.read_line(&mut input) {
        match input.trim().as_ref() {
            "exit" => return,
            _ => {
                input.clear();
                continue;
            }
        }
    }
}
