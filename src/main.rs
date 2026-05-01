use std::sync::mpsc;
use std::thread;

use crate::engine::Engine;
use crate::parser::Event;

mod constants;
mod engine;
mod models;
mod parser;
mod reader;
mod socket_sender;
mod writer;

fn main() {
    let (tx_lines, rx_lines) = mpsc::channel::<String>();
    let (tx_events, rx_events) = mpsc::channel::<Event>();
    let filepath = "testpath.txt";

    /*
    thread::spawn(move || {
        let watcher_result = reader::tail_file(filepath, move |line| {
            let _ = tx_lines.send(line.to_string());
        });

        if let Err(e) = watcher_result {
            eprintln!("Fatal error in watch_file: {}", e);
        }
    });
    */

    let mut shotcall_engine = Engine::new();

    let tick_rate = std::time::Duration::from_millis(50);
    loop {
        match rx_lines.recv_timeout(tick_rate) {
            Ok(line) => {
                if let Some(event) = parser::parse_line(&line) {
                    match event {
                        Event::Interrupt { .. } => shotcall_engine.handle_interrupt(event),
                        Event::CrowdControl { .. } => shotcall_engine.handle_crowd_control(event),
                        Event::Death { .. } => shotcall_engine.handle_death(event),
                        Event::Resurrection { .. } => shotcall_engine.handle_resurrection(event),
                        Event::Other { .. } => shotcall_engine.handle_other(event),
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
        shotcall_engine.process_callouts();
    }

    let test_line = String::from(
        r#"6/14/2025 18:03:27.701-4  SPELL_DAMAGE,Player-1427-0E1E8D11,"Orçaos-Ragnaros-US",0x512,0x20,Creature-0-4218-2661-9671-214668-0003CDF1A1,"Venture Co. Patron",0xa48,0x0,57755,"Heroic Throw",0x1,Creature-0-4218-2661-9671-214668-0003CDF1A1,0000000000000000,117076402,117152954,0,0,42857,0,0,0,1,0,0,0,2643.93,-4917.11,2335,2.4201,80,76552,109359,-1,1,0,0,0,nil,nil,nil,ST"#,
    );
    parser::parse_line(&test_line);
}
