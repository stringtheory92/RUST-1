#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn around(&self) -> u32 {
        self.width * 2 + self.height * 2
    }

    fn double_size(mut self) -> Rectangle {
        self.width = self.width * 2;
        self.height = self.height * 2;
        self
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn print_stuff() {
    let sq = Rectangle::square(100);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // println!("rect1 is {:#?}", rect1);
    // dbg!(&rect1);
    // println!("The area of the rectangle is {} square pixels.", rect1.area());
    // println!("The distance around the rectangle is {} pixels", rect1.around());
    // rect1 = rect1.double_size();
    // println!("The distance around the double-sized rectangle is {} pixels", rect1.around());
    // dbg!(rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can sq hold rect3? {}", sq.can_hold(&rect3));
    
    
}

fn main() {}


