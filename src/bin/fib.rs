fn main() {
    println!("fib(10) = {}", fib(10));
}

fn fib(i: i32) -> i32 {
    match i {
        0 => 0,
        1 => 1,
        i => i + fib(i - 1),
    }
}
