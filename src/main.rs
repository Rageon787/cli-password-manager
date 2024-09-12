use clap::{Args, Parser, Subcommand};
use rand::Rng;

// Constants
const LOWER_ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER_ALPHA: &[u8] = b"ABCDEGHIJKLMNOPQRTSTUVWXYZ";
const NUMERALS: &[u8] = b"1234567890";
const SPECIAL: &[u8] = b"!@#$%^&*()[]{}:;";
const CHARSET: &[&[u8]] = &[LOWER_ALPHA, UPPER_ALPHA, NUMERALS, SPECIAL];

// Command Structs
struct Cli {
    command: Commands,
}

enum Commands {
    Add(add),
    Delete(delete),
    List(list),
    Generate(generate),
}

struct Add {}

struct Delete {}

struct List {}

struct Generate {}

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
    let res: String = generate_password(true, true, true, true, 24);
    println!("{}", res);
}
