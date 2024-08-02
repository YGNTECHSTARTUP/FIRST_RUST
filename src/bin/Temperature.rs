use std::io;
fn main(){
  let mut temp = String::new();
  println!("Enter THe TEmperature in Celisius");
  io::stdin()
  .read_line(& mut temp).expect("Failed TO read the line");
  let temp:u32 = temp.trim().parse().expect("Please Enter THe Number");
 
  let fahrenheit = temp * 9/5 + 32;
  println!("Temerature in Celisius is {} And Temeperature in Fahrenheit is {}",temp,fahrenheit);
}