use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to read line");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Expected to read line");

    let int_input: i32 = input.trim().parse().unwrap();
    let int_input1: i32 = input1.trim().parse().unwrap();

    println!("{}", int_input + int_input1);
}