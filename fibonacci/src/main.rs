fn main() {
    for i in 0..30 {
        println!("fibonacci[{}] = {}", i, create_fibonacci(i));
    }
}

fn create_fibonacci(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => create_fibonacci(n - 1) + create_fibonacci(n - 2),
    }
}
