fn main(){
    let days = ["first","second","third","fourth","fifth","sixth","seventh","eight","ninth","tenth","eleventh","twelenth"];
    let  daysingular = ["","Two","Three","Fourth","Five","Six","Seven","Eight","Nine","Ten","Eleven","Twelve"];
    let  choros: [&str; 12] = ["And A partridge in a pear tree","turtle doves","French Hens","calling Birds","golden rings","geese a layin","swans a swimmin","maids milkin","pipers pipin","ladies dancin","Lords a leapin","drummers drummin"];
   
    let mut script = String::from("");
    for i in 0..12 {
        println!("\nOn The {} day of Christmas",days[i]);
        println!("My true love gave to me");
       let  str =   daysingular[i].to_owned() + " "  + choros[i] + "\n";
       
        script = format!("{}{}",str,script);
        
        println!("{}",script.trim());
    }
}