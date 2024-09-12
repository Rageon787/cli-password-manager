use clap::{Args, Parser, Subcommand};
use rand::Rng;

// Constants
const LOWER_ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER_ALPHA: &[u8] = b"ABCDEGHIJKLMNOPQRTSTUVWXYZ";
const NUMERALS: &[u8] = b"1234567890";
const SPECIAL: &[u8] = b"!@#$%^&*()[]{}:;";
const CHARSET: &[&[u8]] = &[LOWER_ALPHA, UPPER_ALPHA, NUMERALS, SPECIAL];

// Commands
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(Add),
    Delete(Delete),
    List(List),
    Generate(Generate),
}

#[derive(Args)]
struct Add {
    service: Option<String>,
}

#[derive(Args)]
struct Delete {
    service: Option<String>,
}

#[derive(Args)]
struct List {
    service: Option<String>,
}

#[derive(Args)]
struct Generate {
    alpha: Option<String>,
    numeric: Option<String>,
    capital: Option<String>,
    special: Option<String>,
    length: Option<i32>,
    copy: Option<String>,
}

// Helper Functions
fn generate_password(
    alpha: bool,
    capital: bool,
    numeric: bool,
    special: bool,
    length: usize,
) -> String {
    let choices: Vec<usize> = [(alpha, 0), (numeric, 1), (capital, 2), (special, 3)]
        .iter()
        .filter_map(|&(condition, value)| if condition { Some(value) } else { None })
        .collect();
    let mut rng = rand::thread_rng();
    let password = (0..length)
        .map(|_| {
            let i = rng.gen_range(0..choices.len());
            let j = rng.gen_range(0..CHARSET[i].len());
            CHARSET[choices[i]][j] as char
        })
        .collect();
    return password;
}

fn main() {
    println!("{}", generate_password(true, false, true, false, 24));
    let cli = Cli::parse();
    // match &cli.command {
    //     Commands::Add => {}
    //     Commands::Delete => {}
    //     Commands::List => {}
    //     Commands::Generate => {}
    // }
}
