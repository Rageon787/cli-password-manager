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
    username: Option<String>,
    password: Option<String>,
}

impl Add {
    fn print_fields(&self) {
        println!("List: {:?}", self.service);
        println!("List: {:?}", self.username);
        println!("List: {:?}", self.password);
    }
    fn add_backend(args: &Add) {}
}

#[derive(Args)]
struct Delete {
    service: Option<String>,
    username: Option<String>,
}

impl Delete {
    fn print_fields(&self) {
        println!("List: {:?}", self.service);
        println!("List: {:?}", self.username);
    }

    fn delete_backend(args: &Delete) {}
}

#[derive(Args)]
struct List {
    service: Option<String>,
}

impl List {
    fn print_fields(&self) {
        println!("List: {:?}", self.service);
    }
    fn list_backend(args: &List) {}
}

#[derive(Args)]
struct Generate {
    #[arg(short = 'a', long = "alpha", help = "Include alphabetic characters")]
    alpha: bool,

    #[arg(short = 'n', long = "numeric", help = "Include numeric characters")]
    numeric: bool,

    #[arg(short = 'C', long = "capital", help = "Include capital characters")]
    capital: bool,

    #[arg(short = 's', long = "special", help = "Include special characters")]
    special: bool,

    length: Option<i32>,

    #[arg(short = 'c', long = "copy", help = "Copy to clipboard")]
    copy: bool,
}

impl Generate {
    fn print_fields(&self) {
        println!("alpha: {:?}", self.alpha);
        println!("numeric: {:?}", self.numeric);
        println!("special: {:?}", self.special);
        println!("length: {:?}", self.length);
        println!("copy: {:?}", self.copy);
    }

    fn backend(&self) {}
}

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
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(add) => add.print_fields(),
        Commands::Delete(delete) => delete.print_fields(),
        Commands::List(list) => list.print_fields(),
        Commands::Generate(generate) => generate.print_fields(),
    }
}
