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
    let message: &[u8] = b" Hello, World!\n";
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
//on vector 
fn main(){
    let vector : Vec<i32> = vec![32 , 323 , 343 ,44] ;
    let ptr : *const i32 = vector.as_ptr() ;
    unsafe{
        println!("First element is {:?}" , *ptr) ;
        println!("2 nd element is {:?}" , *ptr.add(1)) ;
    }
}


//dont follow this code ...first of all edit this code 

fn main() {
    let vector: Vec<i32> = vec![32, 342, 43];
    let ptr: *const i32 = vector.as_ptr();
    let size: usize = vector.len();

    unsafe {
        let raw_slice: &[i32] = std::slice::from_raw_parts(ptr, size);
        let number_range: std::ops::RangeInclusive<usize> = 0..=size - 1;

        for i in number_range {
            let mut stdout: std::io::Stdout = std::io::stdout();

            // ❌ No safe indexing — pure unsafe raw pointer math
            let value_pointer: *const i32 = raw_slice.as_ptr().add(i);
            let value: i32 = *value_pointer;

            // ✅ Fully stable formatting + byte casting
            let output_string: std::string::String = std::format!(
                "\nIdx is {:?} of value is {:?}",
                i,
                value
            );
            let output_bytes: &[u8] = output_string.as_bytes();

            let _write_result: std::io::Result<()> =
                std::io::Write::write_all(&mut stdout, output_bytes);

            let _flush_result: std::io::Result<()> =
                std::io::Write::flush(&mut stdout);
        }
    }
}


//another simple example
fn main() {
    let array: [i32; 4] = [666, 777, 888, 999];
    let pointer_to_array: *const i32 = array.as_ptr();
    let array_length: usize = array.len();
    let mut stdout_handle: std::io::Stdout = std::io::stdout();
    unsafe {
        let constructed_raw_slice: &[i32] =
            std::slice::from_raw_parts(pointer_to_array, array_length);
        let mut current_index: usize = 0;
        while current_index < constructed_raw_slice.len() {
            // Extreme explicit dereference with pointer arithmetic
            let value_pointer_at_index: *const i32 = constructed_raw_slice.as_ptr().add(current_index);
            let dereferenced_value: i32 = *value_pointer_at_index;
            let output_string: std::string::String = std::format!(
                "Value at index {}: {}\n",
                current_index,
                dereferenced_value
            );
            let output_bytes: &[u8] = output_string.as_bytes();
            let _write_result: std::io::Result<()> = std::io::Write::write_all(
                &mut stdout_handle,
                output_bytes,
            );
            let _flush_result: std::io::Result<()> = std::io::Write::flush(&mut stdout_handle);
            current_index += 1;
        }
    }
}
---------------get_unchecked() is check no bound 
____for loop is safe ...while loop is unsafe and low level 
//another simple example 
 fn main(){
    let array : [i32 ; 5] = [32 , 34 , 43 , 65 ,45] ;
    let pointer_to_array : *const i32 = array.as_ptr() ;
    let array_len : usize = array.len() ;
    unsafe {
        let raw_slice : &[i32] = std::slice::from_raw_parts(pointer_to_array , array_len) ;
        let mut current_index : usize = 0 ;
        while current_index < raw_slice.len(){
            let raw_value_ref : &i32 = raw_slice.get_unchecked(current_index);
            let dref_value : i32 = *raw_value_ref ;
            println!("The idx {:?} of main value {:?}" , current_index , dref_value) ;
            current_index += 1 ;
        }
    }
}

//lets learn mutable pointer in rust
____range of u8 is = 0 to 255.....
fn main(){
    let mut vector : Vec<u8> = vec![32 , 43 , 43 , 53] ;
    let pointer_to_vector : *mut u8 = vector.as_mut_ptr() ;

    unsafe {
        *pointer_to_vector = 10 ;
        println!("Your pointer is {:?}" , *pointer_to_vector) ;

    }
}
//just mutable 
fn main(){
    let mut array : [i32 ; 4] = [32 , 34 , 45 , 54] ;
    let array_size : usize = array.len() ;
    let pointer_to_array : *mut i32 = array.as_mut_ptr() ;
    unsafe {
        let conostructed_raw_slice : &mut [i32] = std::slice::from_raw_parts_mut(pointer_to_array ,array_size) ;
        let mut current_index : usize = 0 ;
        let conostructed_raw_slice_length : usize = conostructed_raw_slice.len() ;
        while current_index < conostructed_raw_slice_length {
            let main_value_from_ptr : &mut i32 = conostructed_raw_slice.get_unchecked_mut(current_index) ;
            *main_value_from_ptr += 100 ;
            let dereferenced_value : i32 = *main_value_from_ptr ;
            let mut stdout : std::io::Stdout = std::io::stdout() ;
            let string_output : std::string::String = std::format!("\nIndex is {:?} of main value {:?}" , current_index ,dereferenced_value) ;
            let output_as_bytes : &[u8] = string_output.as_bytes() ;
            let _result_out : std::io::Result<()> = std::io::Write::write_all(&mut stdout ,output_as_bytes) ;
            let _flushed_out : std::io::Result<()> = std::io::Write::flush(&mut stdout) ;
            current_index += 1 ;
        }
    }
    std::process::exit(0) ;
}
//mutable , immutable , read_only 
fn main(){
    let mut name : std::string::String = String::from("Sohee Al Mahdi Dibbo !") ;
    {
        let slice : &mut str = name.get_mut(0..10).expect("Failed to do it !") ;
        let x : &str = &*slice ;
        println!("Read only data {:?}" , x) ;
        println!("Mutable data {:?}" , slice) ;
    } //x and slice is now out of scope ;
    println!("Your main data is {:?}" , name) ;
}
//____for ownership and borrowing 
fn main() {
    let secret = String::from("dibbo");
    let ref1 = &secret;
    let ref2 = &secret;

    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
    println!("secret: {}", secret); 
}
___________________________________writing 3 simple calculator code in explicit way 
______________________1 => basic explicit 
______________________2 => advance explicit 
______________________3 => extreme explicit 
//so the basic explicit is -
    
fn calculator(number1 : f64 , number2 : f64 , operator : char) -> std::option::Option<f64>{
    match operator{
        '+' => std::option::Option::Some(number1 + number2) ,
        '-' => std::option::Option::Some(number1 - number2) ,
        '*' => std::option::Option::Some(number1 * number2) ,
        '/' => {
            if number2 != 0.0f64{
                std::option::Option::Some(number1 / number2)
            }else{
                std::option::Option::None
            }
        }
        _ => std::option::Option::None ,
    }
}
fn main(){
    let result : std::option::Option<f64> = calculator(2.32 , 33.43 , '/') ;
    let mut stdout : std::io::Stdout = std::io::stdout() ;
    let formated_output : std::string::String = std::format!("Result is {:?}" , result) ;
    let formated_output_to_bytes : &[u8] = formated_output.as_bytes() ;
    let _main_result : std::io::Result<()> = std::io::Write::write_all(
        &mut stdout ,
        formated_output_to_bytes
    ) ;

}
