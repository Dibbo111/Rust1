--How to write hello world in rust ?
//some diffirent ways to write hello world for me !
//A simple way to write hello world using file inix system file named /dev/stdout 
fn main() {
    let mut stdout: std::fs::File = match std::fs::File::create("/dev/stdout") {
        Ok(file) => file,
        Err(error) => {
            // Fully explicit error messaging 
            let mut stderr: std::io::Stderr = std::io::stderr();
            let error_message: String = format!("Error opening /dev/stdout: {}\n", error);
            std::io::Write::write_all(&mut stderr, error_message.as_bytes()).unwrap();
            std::process::exit(1);
        }
    };
    let message: &[u8] = b"Hacker Hello, World!\n";
    match std::io::Write::write_all(&mut stdout, message) {
        Ok(_) => {},
        Err(error) => {
            let mut stderr: std::io::Stderr = std::io::stderr();
            let error_message: String = format!("Error writing to stdout: {}\n", error);
            std::io::Write::write_all(&mut stderr, error_message.as_bytes()).unwrap();
            std::process::exit(1);
        }
    };
}

//Another simple example ...its high level  ; 
//using the io , stdout and flush 
--key : bytes are almost &[u8] in general --
fn main(){
    let mut stdout : std::io::Stdout = std::io::stdout() ;
    let message : &[u8] = b"Hello world" ;
    match std::io::Write::write_all(&mut stdout , message){
        Ok(_) =>{
            println!("  ") ;
        }
        Err(e) => {
            eprintln!("Error as {:?}" ,e) ;
        }
    }
    std::io::Write::flush(&mut stdout).expect("Failed !") ;         //remember that this line must return error 
}

//another simple example of writing hello world 
use std::io;
use std::io::Write;
fn main() -> Result<(), io::Error> {
    let message: &[u8] = b"Hello World!\n";
    let stdout: io::Stdout = io::stdout();
    let mut handle: io::StdoutLock = stdout.lock();
    let mut i: usize = 0;
    let message_len: usize = message.len();
    while i < message_len {
        let start_index: usize = i;
        let end_index: usize = i + 1;
        let slice: &[u8] = &message[start_index..end_index];
        let mut total_written: usize = 0;
        let slice_len: usize = slice.len();
        while total_written < slice_len {
            let write_result: Result<usize, io::Error> = handle.write(&slice[total_written..slice_len]);
            match write_result {
                Ok(bytes_written) => {
                    if bytes_written == 0 {
                        let error: io::Error = io::Error::new(io::ErrorKind::WriteZero, "write returned 0 bytes");
                        return Err(error);
                    }
                    total_written = total_written + bytes_written;
                },
                Err(error) => {
                    return Err(error);
                }
            }
        }
        i = i + 1;
    }
    let flush_result: Result<(), io::Error> = handle.flush();
    match flush_result {
        Ok(()) => {
            return Ok(());
        },
        Err(error) => {
            return Err(error);
        }
    }
}
_____________________________________________hello world done_____________________________________________________
