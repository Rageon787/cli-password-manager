use arboard::Clipboard;
use clap::{Args, Parser, Subcommand};
use rand::Rng;
use rusqlite::{Connection, Result};
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

    fn backend(&self) {
        match &self.service {
            Some(service) => match &self.username {
                Some(username) => match &self.password {
                    Some(password) => todo!(),
                    None => todo!(),
                },
                None => todo!(),
            },
            None => todo!(),
        };
    }
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

    fn backend(&self) {
        match &self.username {
            Some(username) => match &self.service {
                Some(service) => todo!(),
                None => todo!(), // return an error
            },
            None => todo!(), // print a help message
        }
    }
}

#[derive(Args)]
struct List {
    service: Option<String>,
}

impl List {
    fn print_fields(&self) {
        println!("List: {:?}", self.service);
    }
    fn backend(&self) {
        match &self.service {
            Some(service) => todo!(), // print every username associated with this service
            None => todo!(),          // print every service and their usernames
        }
    }
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

    fn generate_password(&self) -> String {
        let choices: Vec<usize> = [
            (self.alpha, 0),
            (self.capital, 1),
            (self.numeric, 2),
            (self.special, 3),
        ]
        .iter()
        .filter_map(|&(condition, value)| if condition { Some(value) } else { None })
        .collect();

        let length = match self.length {
            Some(length) => length,
            None => 12,
        };

        let mut rng = rand::thread_rng();
        let password = (0..length)
            .map(|_| {
                let i = rng.gen_range(0..choices.len());
                let j = rng.gen_range(0..CHARSET[choices[i]].len());
                CHARSET[choices[i]][j] as char
            })
            .collect();

        if self.copy {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(&password);
        }
        return password;
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(add) => add.print_fields(),
        Commands::Delete(delete) => delete.print_fields(),
        Commands::List(list) => list.print_fields(),
        Commands::Generate(generate) => {
            let res = generate.generate_password();
            println!("{}", res);
        }
    }
}
