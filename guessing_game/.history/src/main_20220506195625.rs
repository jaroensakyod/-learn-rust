use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
          println!("Guess the number!");

          let secret_number = rand::thread_rng().gen_range(1..=100);

          println!("The secret number is: {}", secret_number);


          println!("Please input your guess.");

          // --snip--

          let mut guess = String::new();

          io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
          
          let guess:u32 = guess.trim().parent().expect("Please type a number!");

          println!("You guessed: {}", guess);

          match guess.cmp()
}
