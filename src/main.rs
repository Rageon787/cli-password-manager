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

#[derive(Args)]
struct Delete {
    service: Option<String>,
    username: Option<String>,
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

fn add_to_service(service: String, username: String, password: String) {}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(add) => match &add.service {
            Some(service) => match &add.username {
                Some(username) => match &add.password {
                    Some(password) => {
                        // add to (username, password) to service
                        add_to_service(service, username, password);
                    }
                    None => {
                        // Just create entry for service and username
                    }
                },
                None => {
                    // Just create an entry for service
                }
            },
            None => {
                // Print a help message
                println!("Help for add");
            }
        },
        Commands::Delete(delete) => match &delete.service {
            Some(service) => println!("Service {}", service),
            None => println!("No service entered"),
        },
        Commands::List(list) => match &list.service {
            Some(service) => println!("Service {}", service),
            None => println!("no service entered"),
        },
        Commands::Generate(generate) => {
            println!(
                "password is {}",
                generate_password(true, true, true, true, 32)
            );
        }
    }
}
