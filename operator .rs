fn main(){
    let cond = 2 as f32 <= 2.2;
    let cond1 = true && cond;
    let cond2 = !(true && cond);

    println!("{}, {}, {}", cond, cond1, cond2);
}
