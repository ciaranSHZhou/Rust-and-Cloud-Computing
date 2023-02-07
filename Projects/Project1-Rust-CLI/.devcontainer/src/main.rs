//a command line tool to play the game guess the number
use clap::Parser;
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Ciaran Zhou",
    about = "Guess the number game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Ciaran Zhou")]
    Play {
        #[clap(short, long)]
        quote_type: String,
    },
}
// create the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { quote_type }) => {
            let result = quote_of_the_day::quote_of_the_day(&quote_type);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
