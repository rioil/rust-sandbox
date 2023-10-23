fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let c: char = '金';
    println!("{}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    let x = tup.0;
    let z = tup.2;
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    let array = [1, 2, 3, 4, 5];
    println!("The value of array[0] is: {}", array[0]);
}
