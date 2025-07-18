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

------------------------------------------------------------------hello world end --------------------------------
_________________________________________________str____________________________________________________________
//all of string in rust 
fn main(){
    let message : &'static str = "Hello world" ;
    std::println!("{:?}" , message) ;
    std::process::exit(1) ;
    //it will never execute !
}

//another simple example 
fn main(){
    let mut stdout : std::io::Stdout = std::io::stdout() ;
    let message : &'static str = "Hello world and Dibbo !" ;
    match std::io::Write::write_all(&mut stdout , message.as_bytes()){
        Ok(done) => done ,
        Err(error) => {
            let mut stderr : std::io::Stderr = std::io::stderr() ;
            let err_msg : String = format!("Error as {:?}" , error) ;
            std::io::Write::write_all(&mut stderr , err_msg.as_bytes()).unwrap() ;
            std::process::exit(1) ;
        }
    }
}

//for string understand all of how  ptr works ;
fn main() {
    let mut num: i32 = 999;
    let ptr: *mut i32 = &mut num as *mut i32;

    unsafe {
        *ptr = 777;
        println!("Modified value: {}", num);
    }
}
//another simple example 
//use std::ptr;

fn main(){
    let array : [i32  ;3] = [12 , 32 , 43] ;
    let ptr : *const i32 = array.as_ptr() ;
    unsafe {
        let mut stdout : std::io::Stdout = std::io::stdout() ;
        let message : &[u8] = b"The first element of the array is " ;
        std::io::Write::write_all(&mut stdout , message).unwrap() ;
        let value : i32 = std::ptr::read(ptr) ;
        let mut number_buffer = itoa::Buffer::new() ;
        let value_str : &str = number_buffer.format(value) ;
        std::io::Write::write_all(&mut  stdout , value_str.as_bytes()).unwrap() ;

    }
}

//another simple of pointer on array 
fn main() {
    let arr = [666, 777, 888];
    let ptr = arr.as_ptr(); 
    unsafe {
        println!("First value: {}", *ptr);         
        println!("Second value: {}", *ptr.add(1)); 
    }
}
//another simple example 

