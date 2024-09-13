# RustPass 
RustPass is a command line password manager written in Rust and uses sqlite to store PBKDF2 AES encrypted passwords.

This is my first personal project and was intended as a toy project for me to learn rust and get into the habit of making programming projects. 

Please don't use this as your password manager! 

## Features
- Add a new password for a service and multiple accounts.
- Retrieve stored passwords for specific services and copy to them clipboard.
- Generate passwords and copy them to your clipboard. 
- Delete stored passwords. 
- List all services or accounts associated with a particular service
- Passwords are encrypted using AES-256 with the help of a PBKDF2 key derived from the master password. 

## Commands 
**Add a password**  
```bash
rust-pass add <username> <service> <password> 
``` 
**Generate a password** 
```bash
rust-pass generate [-a | --alpha] [-n | --numeric] [-s | --special] [-C | --capital] [length] [-c | --copy] 
```

**Retrieve a password**
```bash 
rust-pass retrieve <username> <service> <password> [-c | --copy] 
```

**Delete a stored password** 
```bash 
rust-pass delete <username> <service> 
```

**List all services or all accounts within a service** 
```bash
rust-pass list [service] 
```
