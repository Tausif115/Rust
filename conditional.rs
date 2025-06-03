use std::io;

fn main(){
    println!("What you want to do?(+,-,*,/):\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error!");

    println!("Enter number 1st numeber: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Error!");

    println!("Enter number 2nd numeber: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Error!");

    let int_input: i32 = input1.trim().parse().unwrap();
    let int_input1: i32 = input2.trim().parse().unwrap();
    let operator = input.trim();

    if operator == "+"{
        let output = int_input + int_input1;
        println!("{} + {} = {}", int_input, int_input1, output);
    }
    else if operator == "-"{
        let output = int_input - int_input1;
        println!("{} - {} = {}", int_input, int_input1, output);
    }
    else if operator == "*"{
        let output = int_input * int_input1;
        println!("{} * {} = {}", int_input, int_input1, output);
    }
    else if operator == "/"{
        let output = int_input / int_input1;
        println!("{} / {} = {}", int_input, int_input1, output);
    }
}
