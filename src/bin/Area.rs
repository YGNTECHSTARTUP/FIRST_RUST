use std::io;
fn main(){
   let mut height = String::from("");
   let mut width = String::from("");
   println!("Enter Height Of the Rectangle:");
   io::stdin().read_line(&mut height).expect("Failed To Read the Line");
   println!("Enter Width Of the Rectangle:");
   io::stdin().read_line(&mut width).expect("Failed To Read the Line");
   let height:u32 = height.trim().parse().expect("Enter the Number");
   let width:u32 = width.trim().parse().expect("Enter The Number");
   let res = calculatearea(height, width);
   println!("Area of the Rectangle of height {} and width {} is {}",height,width,res);
}

fn calculatearea(height:u32,width:u32 )->u32{
   height * width
}