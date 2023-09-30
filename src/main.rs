use rust_learning::greet;
use rand::{thread_rng, Rng};
fn main(){
    greet();
    let x = thread_rng().gen_range(1, 10);
    println!("{}",x);
}