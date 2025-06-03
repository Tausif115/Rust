fn main(){
    let mut tup: (i32, bool, char) = (3, true, 'S');
    //tup.0 = 5;
    tup = (8, false, 'A');

    println!("{}", tup.0);
    println!("{}", tup.2);
    println!("{:?}", tup);
}