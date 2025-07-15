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

//write hello world in raw ay byte by byte 
use std::io::stdout;
use std::io::Write;
use std::io::Result;
fn main() -> Result<()> {
    let mut handle = stdout();
    let message: [u8; 16] = [
        0x48, // 'H'
        0x65, // 'e'
        0x6C, // 'l'
        0x6C, // 'l'
        0x6F, // 'o'
        0x2C, // ','
        0x20, // ' '
        0x77, // 'w'
        0x6F, // 'o'
        0x72, // 'r'
        0x6C, // 'l'
        0x64, // 'd'
        0x21, // '!'
        0x20, // ' '
        0x28, // '('
        0x64, // 'd'
    ];
    let message_extra: [u8; 4] = [
        0x64, // 'd'
        0x29, // ')'
        0x0A, // '\n'
        0x00, // null byte (optional, can skip)
    ];
    let mut index: usize = 0;
    while index < message.len() {
        let byte_slice: &[u8] = &message[index..index+1];
        let write_result = handle.write_all(byte_slice);
        match write_result {
            Ok(()) => (),
            Err(e) => panic!("Write error: {}", e),
        }
        index += 1;
    }
    index = 0;
    while index < message_extra.len() {
        let byte_slice: &[u8] = &message_extra[index..index+1];
        let write_result = handle.write_all(byte_slice);
        match write_result {
            Ok(()) => (),
            Err(e) => panic!("Write error: {}", e),
        }
        index += 1;
    }
    let flush_result = handle.flush();
    match flush_result {
        Ok(()) => (),
        Err(e) => panic!("Flush error: {}", e),
    }
    Ok(())
}

//print hello world using hexdesiml 
use std::process::Command;
use std::string::String;
use std::vec::Vec;
fn main() {
    let cmd_bytes: [u8; 4] = [0x65, 0x63, 0x68, 0x6F]; // "echo"
    let arg_bytes: [u8; 11] = [0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64]; // "hello world"

    // Vec<u8> তে কপি করছি প্রতিটি বাইট
    let mut cmd_vec: Vec<u8> = Vec::new();
    for i in 0..cmd_bytes.len() {
        cmd_vec.push(cmd_bytes[i]);
    }

    let mut arg_vec: Vec<u8> = Vec::new();
    for i in 0..arg_bytes.len() {
        arg_vec.push(arg_bytes[i]);
    }
    let cmd = match String::from_utf8(cmd_vec) {
        Ok(s) => s,
        Err(_) => {
            panic!("Command bytes are not valid UTF-8");
        }
    };
    let arg = match String::from_utf8(arg_vec) {
        Ok(s) => s,
        Err(_) => {
            panic!("Argument bytes are not valid UTF-8");
        }
    };
    let mut command = Command::new(cmd);
    command.arg(arg);
    let output_result = command.output();
    match output_result {
        Ok(output) => {
            let stdout_bytes = output.stdout;
            let stdout_string = match String::from_utf8(stdout_bytes) {
                Ok(s) => s,
                Err(_) => String::from("[Invalid UTF-8 output]"),
            };
            println!("{}", stdout_string);
        }
        Err(e) => {
            panic!("Failed to execute command: {}", e);
        }
    }
}
