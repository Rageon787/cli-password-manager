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

    fn backend(&self, conn: &Connection) {
        match &self.service {
            Some(service) => match &self.username {
                Some(username) => match &self.password {
                    Some(password) => {
                        let check_existing =
                            "SELECT COUNT(*) FROM manager WHERE service = ?1 AND username = ?2";

                        let delete_existing =
                            "DELETE FROM manager WHERE service =?1 AND username = ?2";

                        let insert_new =
                            "INSERT INTO manager (service, username, password) VALUES(?1, ?2, ?3)";
                        let entry_exists: i32 = conn
                            .query_row(check_existing, (service, username), |row| row.get(0))
                            .expect("Failed to check if entry exists");

                        if entry_exists > 0 {
                            conn.execute(delete_existing, (service, username))
                                .expect("Failed to delete existing entry");
                        }
                        match conn.execute(insert_new, (service, username, password)) {
                            Ok(_) => println!("Insertion successful"),
                            Err(e) => eprintln!("Failed to insert into table: {}", e),
                        }
                    }
                    None => eprintln!("Password is missing"),
                },
                None => eprintln!("Username is missing"),
            },
            None => eprintln!("Service is missing"),
        }
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

    fn backend(&self, conn: &Connection) {
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
    fn backend(&self, conn: &Connection) {
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

    fn backend(&self) -> String {
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
    let conn = Connection::open_in_memory().expect("Could not create a connection");
    match &cli.command {
        Commands::Add(add) => add.backend(&conn),
        Commands::Delete(delete) => delete.backend(&conn),
        Commands::List(list) => list.backend(&conn),
        Commands::Generate(generate) => {
            let res = generate.backend();
            println!("{}", res);
        }
    }
}
