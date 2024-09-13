# RustPass 
Presenting you my first ever personal project, A command line password manager written in rust.  
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
