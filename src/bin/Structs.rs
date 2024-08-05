

struct Rectangle {
    height:u32,
    width:u32
}

impl Rectangle {
    fn Square(size:u32) -> Self{
        Self {
            height:size,
            width:size
        }
      
    }
    fn canfit(&self,other:&Rectangle) -> bool {
       self.height > other.height && self.width > other.width
    }
}

fn main() {
    let square = Rectangle::Square(40);
    let rect = Rectangle {
        height:60,
        width:180
    };
   println!("Does Rectangle Can be fitted in the Square:{}",square.canfit(&rect));
   println!("Does Square Can be fitted in the Rectangle:{}",rect.canfit(&square));

}