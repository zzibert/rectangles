#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width >= rect.width && self.height >= rect.height {
            return true
        }  
        else{
            return false
        }   
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 30, height: 60 };
    let rect3 = Rectangle { width: 20, height: 20 };

    println!("can rect1 hold rect2, {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3, {}", rect1.can_hold(&rect3));
}