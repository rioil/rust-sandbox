fn main() {
    println!("Hello, world!");

    print_labeled_measurement(get_measurement(), 'h');
}

/**
 * This function prints a measurement with a label.
 */
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value is {}{}", value, unit_label);
}

/**
 * This function returns a measurement.
 */
fn get_measurement() -> i32 {
    5
}
