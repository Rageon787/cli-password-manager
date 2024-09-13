use aes::{Aes128, Aes256};
use arboard::Clipboard;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use clap::{Args, Parser, Subcommand};
use hmac::Hmac;
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use rand::Rng;
use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};
use std::num::NonZeroU32;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
// Constants
const LOWER_ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER_ALPHA: &[u8] = b"ABCDEGHIJKLMNOPQRTSTUVWXYZ";
const NUMERALS: &[u8] = b"1234567890";
const SPECIAL: &[u8] = b"!@#$%^&*()[]{}:;";
const CHARSET: &[&[u8]] = &[LOWER_ALPHA, UPPER_ALPHA, NUMERALS, SPECIAL];
const MASTER_USERNAME: &str = "masterusername";
const MASTER_PASSWORD: &str = "masterpassword";
const PBKDF2_ITERATIONS: u32 = 100000; // 100k iterations

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
    Retrieve(Retrieve),
}

#[derive(Args)]
struct Retrieve {
    service: Option<String>,
    username: Option<String>,
}

impl Retrieve {
    fn print_fields(&self) {
        println!("List: {:?}", self.service);
        println!("List: {:?}", self.username);
    }

    fn backend(&self, conn: &Connection) {}
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
        match &self.service {
            Some(service) => match &self.username {
                Some(username) => {
                    let check_existing = "SELECT COUNT(*) WHERE service = ?1 AND username = ?2";
                    let entry_exists: i32 = conn
                        .query_row(check_existing, (service, username), |row| row.get(0))
                        .expect("Failed to check if entry exists");

                    if entry_exists > 0 {
                        let delete_existing =
                            "DELETE FROM manager WHERE service = ?1 AND username =?2";
                        match conn.execute(delete_existing, (service, username)) {
                            Ok(_) => println!("Deletion successful"),
                            Err(e) => println!("Failed to delete: {}", e),
                        }
                    } else {
                        println!("errrr")
                    }
                }
                None => eprintln!("Errr"),
            },
            None => eprintln!("errr"),
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

// Cryptographic functions

fn generate_salt() -> Vec<u8> {
    let salt: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen()).collect();
    return salt;
}
fn derive_key(master_password: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0u8; 20];
    pbkdf2_hmac::<Sha256>(master_password.as_bytes(), salt, 100_000, &mut key);
    return key;
}

fn encrypt(master_password: &str, password: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let salt = generate_salt();
    let key = derive_key(master_password, &salt);
    let iv: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen()).collect();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let encrypted_password = cipher.encrypt_vec(password.as_bytes());
    return (salt, iv, encrypted_password);
}

fn decrypt(master_password: &str, salt: &[u8], iv: &[u8], encrypted_password: &[u8]) -> String {
    let key = derive_key(master_password, salt);
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_password = cipher.decrypt_vec(encrypted_password).unwrap();
    return String::from_utf8(decrypted_password).unwrap();
}
fn main() {
    let cli = Cli::parse();
    let conn = Connection::open_in_memory().expect("Could not create a connection");

    let create_table = "CREATE TABLE IF NOT EXISTS manager(
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        service TEXT NOT NULL, 
        username TEXT NOT NULL, 
        password TEXT NOT NULL,  
        UNIQUE(service, username)
    )";

    conn.execute(create_table, ())
        .expect("Failed to create a table");
    match &cli.command {
        Commands::Add(add) => add.backend(&conn),
        Commands::Delete(delete) => delete.backend(&conn),
        Commands::List(list) => list.backend(&conn),
        Commands::Generate(generate) => {
            let res = generate.backend();
            println!("{}", res);
        }
        Commands::Retrieve(retrieve) => retrieve.print_fields(),
    }
}
