use std::{fmt::Debug, string};


#[derive(Debug)]
enum IP {IPV4,IPV6}
#[derive(Debug)]
enum Call {
    phone(String),
    whatsapp(String),
    instagram(String),
}

impl Call {
    fn call (&self,name:&str) {
       println!("Calling {}",name);
       println!("Ringing....");
       println!("{:?}",self);
    }
}

fn main(){
  let number:Option<i32>   = None;
  let phonecall = Call::phone(String::from("8328313651"));
  let whatsappcall = Call::whatsapp(String::from("8328313651ss"));
  let instagramcall = Call::instagram(String::from("cyberboy_ygn"));
  phonecall.call("gagan");
  whatsappcall.call("gagan naidu");
  instagramcall.call("cyberboy");
  let ip4adress = IP::IPV4;
  let ip6adress = IP::IPV6;
  println!("The IpAress is a type of {:?} {:?}",ip4adress,ip6adress);
  
}