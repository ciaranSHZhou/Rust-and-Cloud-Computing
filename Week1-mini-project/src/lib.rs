use rand::Rng;
use std::io;
pub fn guess() {
    //generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        //Prompt the user to guess the number
        println!("Guess the number!");
        //Get user input and convert it to number and store in a variable
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //if the user guesses too high, print "Too high!"
        if guess > secret_number {
            println!("Too high!");
        }
        //if the user guesses too low, print "Too low!"
        if guess < secret_number {
            println!("Too low!");
        }
        if guess == secret_number {
            break;
        }
    }

    //if the user guesses correctly, print "You win!"
    println!("You win!");
}
