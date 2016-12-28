extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
println!("The Guessing Game!");
println!("What's your guess??");
let rand_num = rand::thread_rng().gen_range(1, 101);
loop {
println!("Your Guess: ");
let mut guess_num = String::new();
io::stdin().read_line(&mut guess_num)
.expect("Error: COULD NOT READ LINE");
let guess_num: u32 = match guess_num.trim().parse() {
Ok(num) => num,
Err(_) => continue,
};
println!("You have Guessed: {}", guess_num);
match guess_num.cmp(&rand_num) {
Ordering::Less => println!("Lesser than the secret number"),
Ordering::Greater => println!("Greater than the secret number"),
Ordering::Equal => {
println!("You win!");
break;
}
}
}
}
