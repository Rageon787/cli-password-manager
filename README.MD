# RustPass 

## Features
- Add a new password for a service and multiple accounts.
- Retrieve stored passwords for specific services and copy to them clipboard.
- Generate passwords and copy them to your clipboard. 
- Delete stored passwords. 
- List all services or accounts associated with a particular service
- Passwords are encrypted using AES-256 with the help of a PBKDF2 key derived from the master password. 

## Commands 
- Add a password 
```bash
rust-pass add <username> <service> <password> 
``` 
- Generate a password 
```
