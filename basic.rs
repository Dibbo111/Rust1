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

fn main() {
    let array = [12, 32, 43];
    let ptr = array.as_ptr();

    unsafe {
        print!("The first element of the array is ");
        let value = std::ptr::read(ptr);      //raw copy
        println!("{}", value);
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

//this is advance explicit 
fn calculator(number1: f64, number2: f64, operator: char) -> std::option::Option<f64> {
    match operator {
        '+' => {
            let result_addition: f64 = number1 + number2;
            std::option::Option::Some(result_addition)
        },
        '-' => {
            let result_subtraction: f64 = number1 - number2;
            std::option::Option::Some(result_subtraction)
        },
        '*' => {
            let result_multiplication: f64 = number1 * number2;
            std::option::Option::Some(result_multiplication)
        },
        '/' => {
            if number2 != 0.0f64 {
                let result_division: f64 = number1 / number2;
                std::option::Option::Some(result_division)
            } else {
                std::option::Option::None
            }
        },
        _ => std::option::Option::None,
    }
}

fn main() {
    let number1_input: f64 = 2.32f64;
    let number2_input: f64 = 33.43f64;
    let operator_input: char = '/';

    let result: std::option::Option<f64> = calculator(number1_input, number2_input, operator_input);

    let mut stdout: std::io::Stdout = std::io::stdout();

    match result {
        std::option::Option::Some(value) => {
            let formatted_string: std::string::String = std::format!("✅ Result is: {}\n", value);
            let formatted_bytes: &[u8] = formatted_string.as_bytes();
            let write_result: std::io::Result<()> = std::io::Write::write_all(&mut stdout, formatted_bytes);
            match write_result {
                std::result::Result::Ok(()) => {},
                std::result::Result::Err(e) => {
                    let error_message: std::string::String = std::format!("❌ Write error: {:?}\n", e);
                    let error_bytes: &[u8] = error_message.as_bytes();
                    let _ = std::io::Write::write_all(&mut stdout, error_bytes);
                }
            }
        },
        std::option::Option::None => {
            let error_message: &[u8] = b"❌ Invalid operation or division by zero.\n";
            let write_result: std::io::Result<()> = std::io::Write::write_all(&mut stdout, error_message);
            match write_result {
                std::result::Result::Ok(()) => {},
                std::result::Result::Err(e) => {
                    let error_string: std::string::String = std::format!("❌ Write error: {:?}\n", e);
                    let error_bytes: &[u8] = error_string.as_bytes();
                    let _ = std::io::Write::write_all(&mut stdout, error_bytes);
                }
            }
        }
    }
}
//fizzbuzz 
fn fizzbuzz(number : i32) -> String{
    match (number %3 , number % 5){
        (0, 0) => "FizzBuzz".to_string() ,
        (0 , _) => "Fizz".to_string() ,
        (_ , 0) => "buzz".to_string() ,
        _ => number.to_string()
    }
}
fn main(){
    for i in 1..=10 {
        println!("Result {:?}" , fizzbuzz(i)) ;
    }
}

//another simple of struct 
struct Rectangle{
    index: i32 ,
    age : i32 ,
}
impl Rectangle{
    fn area(&self) -> i32 {
        self.index * self.age
    }
}
fn main(){
    let val = Rectangle{
        index : 32 ,
        age : 323 ,
    } ;
    println!("Area is {:?}" , val.area()) ;
}
//touple struct
struct RGB(i32 , i32 , i32) ;
fn main(){
    let red = RGB(3 , 0 , 10) ;
    let green = RGB(43 , 43 , 9) ;
    let blue = RGB(43 , 654 , 54) ;
    println!("The num of red {:?} {:?} {:?}" , red.0 , red.1 , red.2) ;
    println!("The num of green {:?} {:?} {:?}" ,green.0 , green.1 , green.2) ;
    println!("The num of blue {:?} {:?} {:?}" , blue.0 , blue.1 , blue.2) ;

}

//another simple example of touple struct  of explicit string lifetime annotation 
struct Credential(&'static str , &'static str) ;
fn main(){
    let cred = Credential("admin" , "passwd") ;
    println!("The user is {:?} and the passowrd is {:?}" , cred.0 , cred.1) ;

}
//with a explicit type 
struct Credential(&'static str , &'static str) ;
fn main(){
    let cred : Credential = Credential("KOHEE" , "1234") ;
    println!("The username is {:?} and the password is {:?}" , cred.0 , cred.1) ;
}
//Just added impl 
struct Credential(&'static str , &'static str) ;
impl Credential{
    fn display(&self){
        println!("The user name is {:?} and the password is {:?}" ,self.0 , self.1) ;
    }
}
fn main(){
    let cred : Credential = Credential("kohee", "12345") ;
    cred.display() ;
}

//using a custom constructor struct Credential(&'static str , &'static str) ;
impl Credential{
    fn new(username :&'static str , password :&'static str) -> Credential{
        Credential(username , password)
    }
    fn display(&self){
        println!("The username is {:?} and the password is {:?}" ,self.0 , self.1) ;
    }
}
fn main(){
    let cred : Credential = Credential::new("Kohee" , "1234") ;     //Credential::new() for the custom constructor ; 
    cred.display() ;
}

//another simple example of magnitude .....using vector ....vector for pHyscis ...not like std::vec::Vector<T> 
struct Vector2d(f64, f64);

impl Vector2d {
    fn new(value1: f64, value2: f64) -> Vector2d {
        Vector2d(value1, value2)
    }
    fn magnitude(&self) -> f64 {
        ((self.0 * self.0) + (self.1 * self.1)).sqrt()
    }
    fn display(&self) {
        println!("The Vector is ({}, {})", self.0, self.1);
        println!("Magnitude is {}", self.magnitude());
    }
}
fn main() {
    let vec: Vector2d = Vector2d::new(32.0, 33.2);
    vec.display();
}

//another simple  example of stack using impl and struct 
struct Stack{
    data : [i32 ; 10] ,
    index : usize ,
}
impl Stack{
    fn new() -> Stack{
        Stack {data : [0 ; 10] , index : 0}
    }
    fn push(&mut self , value : i32){
        if self.index  < 10 {
            self.data[self.index] = value ;
            self.index = self.index + 1 ;
        }else{
            println!("Stack overflow ! value {:?}" , value) ;
        }
    }
    fn pop(&mut self) -> i32 {
        if self.index == 0 {
            println!("Stack underflow !") ;
            std::process::exit(1) ;
        }
        self.index = self.index - 1 ;
        self.data[self.index]
    }
    fn print_stack(&self){
        for i in 0..self.index{
            if i > 0{
                print!(" ,") ;
            }
            println!("Stacked value is {:?}" ,self.data[i]) ;
        }
    }
}
fn main(){
    let mut stack : Stack = Stack::new() ;
    for i in 1..5{
        stack.push(i * 2 + 1) ;
    }
    stack.print_stack() ;
    let popped : i32 = stack.pop() ;
    println!("popped value is {:?}" , popped) ;
    println!("After pop !") ;
    stack.print_stack() ;

    //underflow checking
    for _i in 0..6{
        let pop : i32 = stack.pop() ;
        println!("Popped value but underflow as {:?}" , pop)  ;
    }
    stack.print_stack() ;
    //overflow checking
    for i in 1..20 {
        stack.push(i * 10) ;
    }
    stack.print_stack() ;
}
//another simple example of counting ...using struct , impl and Option type 
struct Counter{
    number : i32 ,
    limit : i32 ,
}
impl Counter{
    fn new(number : i32 , limit : i32)-> Counter{
        Counter{number , limit}
    }
    fn count(&mut self){
        while self.number < self.limit {
            self.number = self.number + 1 ;
            println!("Num : {:?}" , self.number) ;
            if self.number == self.limit{
                eprintln!("Number limit reached {:?}" , self.number) ;
                std::process::exit(1) ;
            }
        }
    }
    fn get(&self) -> Option<i32>{
        if self.number <= self.limit{
            Some(self.number)
        }else{
            None
        }
    }
}
fn main(){
    let mut count : Counter = Counter::new(2 , 34) ;
    count.count() ;
    match count.get(){
        Some(number) => {
            println!("Number {:?}" , number) ;
        }
        None => println!("No number found") ,
    }

}

//another simple example returning result 
struct Counter{
    number : i32 ,
    limit : i32 ,
}
impl Counter{
    fn new(number : i32 , limit : i32) -> Counter{
        Counter{number , limit}
    }
    fn count(&mut self){
        while self.number < self.limit{
            self.number = self.number + 1 ;
            println!("Your number is {:?}" , self.number) ;
            if self.number == self.limit{
                eprintln!("Number limit reached at {:?}" , self.number) ;
                std::process::exit(1) ;
            }
        }
    }
    fn get(&self) -> Result<i32 , String>{
        if self.number <= self.limit{
            Ok(self.number)
        }else{
            Err("Limit reached ....no valid number !".to_string())
        }
    }
}
fn main(){
    let mut count = Counter::new(0 , 10) ;
    count.count() ;
    match count.get(){
        Ok(num) => println!("{:?}" , num) ,
        Err(e) => eprintln!("Error {:?}" , e) ,
    }
}
