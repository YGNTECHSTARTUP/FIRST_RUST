use std::{io, u32};
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Number Kanipettu Mawa");
    loop{
        let mut guess = String::new();
   
        io::stdin().read_line(&mut guess).expect("Edokka Number Kottu Mawa ");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
       
        match guess.cmp(&secret_number){
          
            Ordering::Greater =>  println!(" {guess} kanna Chinna  Number Mawa"),
               
               
            Ordering::Less => println!("{guess} kanna pedha Number Mawa"),
            Ordering::Equal => {
                println!("You Win Mawa");
                break;
            },
        };
    }
    
}