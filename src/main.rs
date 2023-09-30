fn main() {
    let x=4;
    {
        let x=3;
        println!("x is {}",x);
    }
    println!("x is {}",x);
    // println!("Hello, world!");
}
