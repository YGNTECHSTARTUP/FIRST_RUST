
struct Rect {
    height:i32,
    width:i32
}

impl Rect {
    fn comp(&self,other:&Rect) -> bool {
       self.height < other.width && self.width < other.width
    }
}

fn main(){
     let rect1 = Rect {
        height:20,
        width:40
     };
     let rect2 = Rect {
        height:40,
        width:60
     };
     let rect3 = Rect {
        height:60,
        width:80
     };
    println!("Does Rect 2 can be fitted in rect1:{}",rect1.comp(&rect2));
    println!("Does Rect 2 can be fitted in rect1:{}",rect2.comp(&rect1));
    println!("Does Rect 2 can be fitted in rect1:{}",rect3.comp(&rect2));
}