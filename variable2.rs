fn main(){
    let x = 5;
    println!("x is: {}", x);

    {
        let x = x + 1;
        println!("x is: {}", x);
    }
    
    let x = x - 1;
    println!("x is: {}", x);

    let x = "Hello";
    println!("x is: {}", x);
}