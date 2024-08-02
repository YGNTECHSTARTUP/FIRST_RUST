use std::io;
fn main(){
    let mut n = String::new();
    let mut i = 1;
    println!("Enter THe Value");
    io::stdin().read_line(&mut n).expect("Failed To Read The LIne");
    let n = n.trim().parse().expect("Expected Number");
    while i<n {
        if i == 1 {
            println!("{i}");
        }
        if i>1{
            i = i-1 + i+2;
           println!("{i}")
        }
        i+=1;
    }
}