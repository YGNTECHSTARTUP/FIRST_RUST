

enum Coin {
    Cent,
    Ruppe,
    Euro,
    Dollor,
}

enum Die {
    One,Two,Three,Four,Five,Six
}

fn main() {
    let die_value = Die::One;
    match die_value {
        Die::Six => println!("Another Chance"),
        _ => ()
    }
    let  value = CoinValue(Coin::Ruppe);
    println!("The Value of the Coin is {}",value);
   let value = coinIncrement(Some(6));
    println!("The Value Of the Coin is {:?}",value);
}


fn coinIncrement (coin:Option<i32>) -> Option<i32> {
    match coin {
        None => None,
        Some(i) => Some(i)
    }
}

fn CoinValue (coin:Coin) -> i32 {
      match coin {
        Coin::Cent => 1,
        Coin::Euro => 2,
        Coin::Ruppe => 3,
        Coin::Dollor => 4
      }
}