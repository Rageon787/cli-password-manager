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
}

// Helper Functions
fn generate_password(
    alpha: bool,
    capital: bool,
    numeric: bool,
    special: bool,
    length: usize,
) -> String {
    let mut choices: Vec<usize> = Vec::new();

    if alpha == true {
        choices.push(0);
    }

    if capital == true {
        choices.push(1);
    }
    if numeric == true {
        choices.push(2);
    }

    if special == true {
        choices.push(3);
    }

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
    let _cli = Cli::parse();
}
