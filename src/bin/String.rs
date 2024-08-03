use std::io;
fn main(){
  let mut str= String::new();
  println!("Enter The String");
  io::stdin().read_line(&mut str).expect("Failed TO Read");
  let str = string_parser(&str);
  print!("{str}");
}

fn string_parser (str:&String) -> &str {
   let bytes = str.as_bytes();
   for (i,&item) in bytes.iter().enumerate(){
    println!("On Index {} the Byte is {}",i,item);
    if item == 32 {
        return &str[0..i];
    }
   }
   return &str[..];
}
