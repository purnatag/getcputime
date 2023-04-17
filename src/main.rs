//use std :: time :: Duration;
use cpu_time :: ThreadTime;

fn main() {
    let start_time = ThreadTime :: now();
    for n in 1..101{
        fizzbuzz(n);
    }
    println!("Hello Cpu_Time!");
    let cpu_time = start_time.elapsed();
    println!("{:?}", cpu_time)    
}

fn fizzbuzz(n: u32) -> () {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
