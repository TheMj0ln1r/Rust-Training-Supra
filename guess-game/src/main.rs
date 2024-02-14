use std::io; // bring io library into scope
use rand::Rng; // random library // What exactly the syntax mean?
fn main() {
    let mut chances = 5;
    let rand_number: u8 = rand::thread_rng().gen_range(1..100); // Why we are using rand::___ . Why we are not use rand::Rng
    while chances != 0{
        println!("Enter your guess : ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed"); // we can use std::io::__ why this works

        let number: u8 = number.trim().parse().expect("Not a number..!");

        if number >= 1 && number < 100 {
           if number > rand_number{
            println!("Too High..");
            chances -= 1;
            println!("You have {} chances left.", chances);
           }
           else if number < rand_number{
            println!("Too low..");
            chances -= 1;
            println!("You have {} chances left.", chances);
           }
           else{
            println!("Correct..!");
            break;
           }
        }
    }

}
