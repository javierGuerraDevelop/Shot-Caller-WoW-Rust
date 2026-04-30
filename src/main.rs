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
    /*
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
    */
    let test_line = String::from(
        r#"6/14/2025 18:03:27.701-4  SPELL_DAMAGE,Player-1427-0E1E8D11,"Orçaos-Ragnaros-US",0x512,0x20,Creature-0-4218-2661-9671-214668-0003CDF1A1,"Venture Co. Patron",0xa48,0x0,57755,"Heroic Throw",0x1,Creature-0-4218-2661-9671-214668-0003CDF1A1,0000000000000000,117076402,117152954,0,0,42857,0,0,0,1,0,0,0,2643.93,-4917.11,2335,2.4201,80,76552,109359,-1,1,0,0,0,nil,nil,nil,ST"#,
    );
    parser::parse_line(&test_line);
}
