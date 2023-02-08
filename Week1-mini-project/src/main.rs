//a command line tool to play the game guess the number
use clap::Parser;
#[derive(Parser)]
#[clap(version = "1.0", author = "Ciaran Zhou", about = "Guess the number game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Ciaran Zhou")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}
// create the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            println!("Hello, {}!", name);
            guess::guess();
        }
        None => println!("No command was used"),
    }
}





