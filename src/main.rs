use std::sync::mpsc;
use std::thread;

mod constants;
mod engine;
mod models;
mod parser;
mod reader;
mod socket_sender;
mod writer;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let filepath = "testpath.txt";

    thread::spawn(move || {
        let watcher_result = reader::tail_file(filepath, move |line| {
            let _ = tx.send(line.to_string());
        });

        if let Err(e) = watcher_result {
            eprintln!("Fatal error in watch_file: {}", e);
        }
    });

    for line in rx {
        parser::parse_line(&line);
    }
}
