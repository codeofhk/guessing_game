use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game");
    println!("Enter the value");

    let random = rand::thread_rng().gen_range(0..=100);

    println!("random number : {random}");

    loop{

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess:u32 = match guess
                    .trim()
                    .parse(){
                        Ok(num) => num,
                        Err(_) => {println!("please enter a number");continue;},
                    };

        match guess.cmp(&random){
            Ordering::Less => println!("predicted less"),
            Ordering::Greater => println!("predicted more"),
            Ordering::Equal => {println!("you win");break},
        }
    }

}
