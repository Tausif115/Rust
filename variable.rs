fn main(){
    let mut x = 5;
    let y = 9;
    println!("{} + {} = {}", x, y, x+y);

    {
        let x = x + 1;
        println!("{} + {} = {}", x, y, x+y);
    }

    x = 9;
    println!("{} + {} = {}", x, y, x+y);
    let y = 10;
    println!("{} + {} = {}", x, y, x+y);
}