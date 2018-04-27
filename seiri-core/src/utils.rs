use std::io;
use seiri::Bang;
use seiri::database::query_tracks;
use rusqlite::Connection;

pub fn wait_for_exit(conn: &Connection) {
    let stdin = io::stdin();
    println!("Type 'exit' to exit");

    let mut input = String::new();
    while let Ok(_) = stdin.read_line(&mut input) {
        if input.trim().eq_ignore_ascii_case("exit") {
            return;
        }
        if input.trim().starts_with("query") {
            let query_str: &str = match input.trim().splitn(2, " ").nth(1) {
                Some(query_str) => query_str,
                None => "",
            };

            match Bang::new(query_str) {
                Ok(bang) => {
                    println!("{:?}", bang);
                  //  println!("Compiles to... -------------");
                    let tracks = query_tracks(bang, conn, None, None);
                    println!("{:?}", tracks)
                },
                Err(err) => println!("{:?}", err),
            }
        }
        input.clear();
        continue;
    }
}
