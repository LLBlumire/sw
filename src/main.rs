mod traits;
mod dice;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::io::Read;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;

fn main() {
    let mut file_path = args().skip(1).next().unwrap_or_else(|| {
            exit(1)
    });
    let input: String = {
        let mut file_in = File::open(&file_path).unwrap_or_else(|_| {
            exit(1)
        });
        let mut buffer = String::new();
        file_in.read_to_string(&mut buffer).unwrap_or_else(|_| {
            exit(1)
        });
        buffer
    };

    while let Some(command) = await_command(&file_path) {

    }

}

fn await_command<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut stdout = stdout();
    stdout.write(format!("{}> ", path.as_ref().to_string_lossy()).as_bytes()).ok()?;
    stdout.flush().ok()?;
    let mut stdin = stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok()?;
    if buf.trim() == "exit" {
        None
    } else {
        Some(buf.trim().to_string())
    }
}