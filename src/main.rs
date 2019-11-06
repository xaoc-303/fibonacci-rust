use std::env;
use std::time::Instant;
mod fibo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f: String = args[1].to_string();
    let n: u128 = args[2].parse::<u128>().unwrap();

    println!("{}", start(f, n));
}

fn start(f: String, n: u128) -> (u128) {
    if f == "f1" {
        let start = Instant::now();
        let result = fibo::f1(n);
        let elapsed = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", elapsed);
        println!("Time elapsed in expensive_function() is: {:.8}", elapsed.as_secs_f64() );
        return result;
    }

    if f == "f2" {
        let start = Instant::now();
        let result = fibo::f2(n);
        let elapsed = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", elapsed);
        println!("Time elapsed in expensive_function() is: {:.8}", elapsed.as_secs_f64() );
        return result;
    }

    return 0;
}
