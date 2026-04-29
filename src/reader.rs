use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn tail_file<P, F>(filepath: P, mut on_line: F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(&str),
{
    let file = File::open(&filepath)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => {
                thread::sleep(Duration::from_millis(100));
            }
            Ok(_) => {
                println!("Print: {}", line);
                on_line(&line);
                line.clear();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
