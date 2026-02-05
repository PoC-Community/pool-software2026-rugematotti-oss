use clap::{Parser, Subcommand};

mod models;
mod crypto;
mod input;



use crate::models::{PasswordEntry, PasswordStore};
use crate::crypto::CryptoManager;
use crate::input::password;
use base64::{engine::general_purpose, Engine as _};

#[derive(Parser)]
#[command(name = "pwdmgr", version, about = "Simple password manager")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        service: String,
        username: String,
    },
    List,
    Generate {
        length: u32,
    },
}

fn main() {
    let args = Args::parse();
    let mut store = PasswordStore {
    entries: Vec::new(),
};
    let master_password = password();
    let crypto = CryptoManager::new(&master_password);

    match args.command {
        Commands::Add {service,username,} => {
            println!("Enter password for {}:", service);
            let user_password = password(); // gets raw string
            let encrypted_password_bytes = crypto.encrypt(&user_password);
            let encrypted_password =general_purpose::STANDARD.encode(&encrypted_password_bytes);


            let entry = PasswordEntry {
                service,
                username,
                password: encrypted_password,
            };
            store.entries.push(entry);
            println!("Password added.");
        }

        Commands::List => {
            if store.entries.is_empty() {
                println!("No passwords stored yet.");
            } else {
                for entry in &store.entries {
                    println!("{} → {}", entry.service, entry.username);
                }
            }
        }
        Commands::Generate { length } => {
            println!("Generate password of length {}", length);
        }

        
    }
}