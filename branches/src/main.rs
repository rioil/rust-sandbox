fn main() {
    let number = 3;
    check_number(number);
    let number = 7;
    check_number(number);
}

fn check_number(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
