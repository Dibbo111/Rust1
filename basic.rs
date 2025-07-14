--How to write hello world in rust ?
//some diffirent ways to write hello world for me !
use std::fs::File;
use std::io::Write;
use std::process::exit;
fn main() {
    // /dev/stdout হলো Unix / linux  এর standard output device
    let mut stdout = match File::create("/dev/stdout") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening /dev/stdout: {}", e);
            exit(1);
        }
    };
    let message = b"Hacker Hello, World!\n";
    match stdout.write_all(message) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error writing to stdout: {}", e);
            exit(1);
        }
    };
}
