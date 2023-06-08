fn main() {
    println!("Hello, world!");

    another_function(get_value(), 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("The value is {}{}", value, unit_label);
}

fn get_value() -> i32 {
    5
}
