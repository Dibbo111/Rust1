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


