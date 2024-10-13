mod module;

use crate::module::nice_to_meet_you;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,

    /// The greeting message to use
    #[arg(short, long, default_value = "Hello")]
    greeting: String,
}

fn main() {
    let args = Args::parse();

    println!("{}, {}!", args.greeting, args.name);
    nice_to_meet_you();
}
