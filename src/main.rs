fn main() {
    let x=4;
    let mut y=5;
    {
        let x=3;
        println!("x is {}",x);
        println!("y is {}",y);
    }
    println!("x is {}",x);
    println!("y is {}",y);
    y=6;
    println!("y is {}",y);
    println!("Hello, world!");
}
