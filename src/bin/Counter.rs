fn main() {
   
      let mut counter = 0;
      let countupgrade = 'firstloop:loop{
        counter = counter +1;
        println!("Counter Upgraded By 1 Counter={}",counter);
        if counter == 10 {
            loop{
                println!("Counter Upgraded By 2 Counter={}",counter);
                counter+=2;
                if counter == 20 {
                    break;
                }
            }
       break  'firstloop counter;    
        }
      
      };
      
      println!("CounterUpgrade:{}",countupgrade);
   
   
}