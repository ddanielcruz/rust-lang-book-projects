#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rec1 = Rectangle {
        width: dbg!(30 * scale),
        height: 100,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    // dbg!(&rec);
    println!("rec is {rec1:#?}");

    let rec2 = Rectangle::square(10);
    println!("rec1 can hold rec2? {}", rec1.can_hold(&rec2));
}

fn _area_object(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}

fn _area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
