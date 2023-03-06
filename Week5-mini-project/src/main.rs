//cli that use lib.rs to return all the prime numbers between 1 and the number given by the user
use clap::Parser;
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Ciaran Zhou",
    about = "All prime numbers between 1 and the number given by the user"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}
//create a struct to store the number given by the user
#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Ciaran Zhou")]
    Play {
        #[clap(short, long)]
        num: i32,
    },
}
//create the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { num }) => {
            println!("The prime numbers between 1 and {} are:", num);
            for i in prime_numbers::prime_numbers(num) {
                println!("{}", i);
            }
        }
        None => println!("No command was used"),
    }
}