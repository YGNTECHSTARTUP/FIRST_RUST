struct Rect (u32,u32);
#[derive(Debug)]
struct User {
    name:String,
    age:i32
}

impl  Rect {
    fn area (&self) {
        println!("Area of The Rectangle of height {} and width {} is {}",self.0,self.1,self.0 * self.1);
    }
}

fn main(){
   let rectangle = Rect (40,40);
   area(&rectangle);
   let user = User {
    name:String::from("Gagan"),
    age:17
   };
   println!("User Structure looks like {:#?}",user);
   rectangle.area();
}
fn area(rectangles:&Rect){
    let res = rectangles.0 * rectangles.1;
    println!("Area Of Rectange of height {} and width {} is {}",rectangles.0,rectangles.1,res);
}