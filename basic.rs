

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
//another simple example 
//colour inverting theoreme ---------->    Inverted Component=255−Original Component
struct Colour{
    red : u32 , 
    green : u32 , 
    blue : u32 ,
}
impl Colour{
    fn new(red : u32 , green : u32 , blue : u32) -> Colour{
        Colour{red , green , blue}
    }
    fn invert(&mut self){
        self.red = 255 - self.red  ; 
        self.green = 255 - self.green  ; 
        self.blue = 255 - self.blue ; 
    }
}
fn main(){
    let mut colour : Colour = Colour::new(33 , 22 , 43) ; 
    println!("orginal colour -> red {:?} green {:?} blue {:?}" , colour.red , colour.green , colour.blue) ; 
    colour.invert() ;
    println!("After invert -----> red {:?} green {:?} blue {:?}" , colour.red , colour.green , colour.blue) ; 
}
//another simple example of a timer 
use std::{thread , time} ; 

struct Timer{
    sec : i32 ,
}
impl Timer{
    fn new(sec : i32)-> Timer{
        Timer{sec}
    }
    fn tick(&mut self){
        if self.sec > 0 {
            self.sec = self.sec - 1 ; 
        }
    }
    fn is_finished(&self)-> bool{
        self.sec == 0 
    }
}
fn main(){
    let mut  timer : Timer = Timer::new(5) ; 
    println!("Timer has set for {:?} second !" , timer.sec) ; 
    while !timer.is_finished(){
        timer.tick() ; 
        thread::sleep(time::Duration::from_secs(1)) ; 
        println!("Time left {:?} second !" , timer.sec) ;
    }
    println!("Timer finished !") ;
} 
//another simple example of vector 
fn main(){
    let vec : std::vec::Vec<std::vec::Vec<i32>> = std::vec::Vec::from([
        std::vec::Vec::from([1 , 2 ,3]) ,
        std::vec::Vec::from([2 , 3 , 4]) ,
        std::vec::Vec::from([3 , 4 , 5]) ,
    ]) ;
    for i in &vec{
        println!("Vector is {:?}" , i) ;
    }
}

//just reversing 
fn main(){
    let mut vector : std::vec::Vec<std::vec::Vec<i32>> = std::vec::Vec::new() ;
    let mut count = 0 ;
    for _i in 0..3{
        let mut row : std::vec::Vec<i32> = std::vec::Vec::new() ;
        for _j in 0..3{
            row.push(count) ;
            count = count + 1 ;
        }
        vector.push(row) ;
    }
    println!("Printing vector _____") ;
    for i in 0..vector.len(){
        for j in 0..3{
            print!("{:?}" , vector[i][j]) ;
        }
        println!("") ;
    }
    println!("Reversed vew !") ;
    for i in (0..3).rev(){
        for j in (0..3).rev(){
            print!("{:?}" , vector[i][j]) ;
        }
        println!("") ;
    }
}
//another simple example to find the largest number 
fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    let mut max = matrix[0][0];
    for i in 0..3 {
        for j in 0..3 {
            if matrix[i][j] > max {
                max = matrix[i][j];
            }
        }
    }
    println!("Max number = {}", max); 
}
//just increase value 
fn main(){
    let mut vector :[[i32 ; 3] ; 3] = [[0 ; 3] ; 3] ;
    let mut count = 1 ;
    for i in 0..3{
        for j in 0..3{
            vector[i][j] = count ;
            count = count + 1 ;
        }
    }
    for i in 0..3{
        for j in 0..3{
            print!("{:?}" , vector[i][j]) ;
        }
        println!() ;
    }
}
//this code will be valid when row index == column index
fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("Main Diagonal:");
    for i in 0..3 {
        println!("{}", matrix[i][i]); 
    }
}
//secondary diagonal 
fn main(){
    let matrix : [[i32 ; 3] ; 3] = [
        [1 , 2 , 3] ,
        [3 , 4 , 5]  ,
        [6 , 7 , 8] ,
    ] ;
    for i in 0..3{
        println!("Secondary diagonal {:?}" , matrix[i][2 - i]) ;
    }
}
//the sum of secondary diagonal 
fn main(){
    let matrix : [[i32 ; 3] ; 3] = [
        [1 , 2 , 3] ,
        [4 , 5 , 6] ,
        [7 , 8 , 9] ,
    ] ;
    let mut sum = 0 ;
    for i in 0..3{
        sum = sum + matrix[i][2 - i] ;
    }
    println!("The sum is {:?}" , sum) ;
}
//for collum wise addition 
fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    for j in 0..3 {
        let mut col_sum = 0;
        for i in 0..3 {
            col_sum += matrix[i][j];
        }
        println!("Column {} sum = {}", j, col_sum);
    }
}
//using while loop....building logic...adding number row wise and finding the bigest number form the sum 
fn main(){
    let matrix : [[i32 ; 3] ; 3] = [
        [32 , 432 , 42] ,
        [54 , 43 , 65] ,
        [54 , 32, 43] ,
    ];
    let mut  row_sum : [i32 ; 3] = [0 ; 3] ;
    let mut i = 0 ;
    while i < 3{
        let mut j = 0 ;
        while j < 3{
            row_sum[i] = row_sum[i] + matrix[i][j] ;
            j = j + 1 ;
        }
        i = i + 1 ;
    }
    let mut index : usize = 0 ;
    while index < 3 {
        println!("The sum is {:?} of row {:?}" , row_sum[index] , index) ;
        index = index + 1 ;
    }
    let mut max_sum = row_sum[0] ;
    let mut max_row = 0 ;
    let mut k = 1 ;
    while k < 3{
        if row_sum[k] > max_sum{
            max_sum = row_sum[k] ;
            max_row = k ;
        }
        k = k + 1 ;
    }
    println!("The max sum is {:?} in row {:?}" , max_sum , max_row) ;
}
//another simple example find the min and max number with position 
fn main(){
    let matrix : [[i32 ; 3] ; 3] =[
        [32 , 32 , 43] ,
        [54 , 53 , 22] ,
        [89 , 42 , 22] ,
    ] ;
    let mut min_value = matrix[0][0] ;
    let mut max_value = matrix[0][0] ;
    let mut max_position = (0 , 0) ;
    let mut min_position = (0 , 0) ;
    for i in 0..3{
        for j in 0..3{
            if matrix[i][j] > max_value{
                max_value = matrix[i][j] ;
                max_position = (i , j) ;
            }
            if matrix[i][j] < min_value{
                min_value = matrix[i][j] ;
                min_position = (i , j) ;
            }
        }
    }
    println!("The max value is {:?} of position {:?}" , max_value , max_position) ;
    println!("The min value is {:?} of position {:?}" , min_value  ,min_position) ;
}
//reverse the row of the matrix 
fn main(){
    let mut  array : [[i32 ; 3] ; 3] = [
        [1 , 2 , 3 ] ,
        [3 , 4 , 5]  ,
        [6 , 7 , 8]  ,
    ];
    for i in 0..3{
        array[i].reverse() ;
    }
    for i in 0..3{
        for j in 0..3{
            print!("{:?}" , array[i][j]) ;
        }
        println!("") ;
    }
}
//transposing ____
fn main(){
    let array = [
        [1 , 2 , 3] ,
        [4 , 5 , 6] ,
        [7 , 8 , 9] ,
    ] ;
    let mut transpose : [[i32 ; 3] ; 3] = [[0 ; 3] ; 3] ;
    for i in 0..3{
        for j in 0..3 {
             transpose[j][i] = array[i][j] ;
        }
    }
    println!("Transposed !") ;
    for i in 0..transpose.len(){
        for j in 0..3{
            print!("{:?} " , transpose[i][j]) ;

        }
        println!("") ;
    }
}
//using struct  and impl for matrix 
struct Matrix{
    data : [[i32 ; 3] ; 3] ,
}
impl Matrix{
    fn new(data : [[i32 ; 3] ; 3]) -> Matrix{
        Matrix{data : data}
    }
    fn display(&self){
        for row in 0..3{
            for coll in 0..3{
                print!("{:?} " , self.data[row][coll]) ;
            }
            println!() ;
        }
    }
}
fn main(){
    let mtx = Matrix::new([[32  , 43 , 22 ] , [43 , 322 , 54] , [78 , 67 , 5465]]) ; 
    mtx.display() ;
}

//i skeep this matrix multiplication ...to difiicult for me 
struct Matrix2x2 {
    data: [[i32; 2]; 2],
}

impl Matrix2x2 {
    fn new(data: [[i32; 2]; 2]) -> Matrix2x2 {
        Matrix2x2 { data }
    }

    fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        let mut result = [[0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                result[i][j] = 0;
                for k in 0..2 {
                    result[i][j] = result[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix2x2::new(result)
    }

    fn display(&self) {
        for row in 0..2 {
            for col in 0..2 {
                print!("{} ", self.data[row][col]);
            }
            println!();
        }
    }
}

fn main() {
    let m1 = Matrix2x2::new([[1, 2], [3, 4]]);
    let m2 = Matrix2x2::new([[2, 0], [1, 2]]);
    let product = m1.multiply(&m2);
    product.display();
}
