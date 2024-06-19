#[derive(Debug)] // output debugging information
struct Rectangle{
    width: u64,
    height: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 20
    };
    println!("The are of rectangle is {}", rect1.area());
    println!("rect1 is {rect1:#?}")
}

