struct Rect (u32,u32);
fn main(){
   let rectangle = Rect (40,40);
   area(rectangle);
}
fn area(rectangles:Rect){
    let res = rectangles.0 * rectangles.1;
    print!("Area Of Rectange of height {} and width {} is {}",rectangles.0,rectangles.1,res);
}