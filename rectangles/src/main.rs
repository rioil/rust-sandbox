fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect1,
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    check_can_hold(&rect1, &rect2);

    let rect2 = Rectangle::square(50);
    check_can_hold(&rect1, &rect2);
}

fn check_can_hold(rect1: &Rectangle, rect2: &Rectangle) {
    println!(
        "Can {:?} hold {:?}? {}",
        rect1,
        rect2,
        rect1.can_hold(rect2)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
