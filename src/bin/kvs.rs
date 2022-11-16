use std::env::current_dir;
use std::option::Option::Some;
use std::process::exit;
use clap::{Arg, Command};
use kvs::{KvStore, Result, KvsError};

fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to string")
                .arg(Arg::new("KEY").required(true).help("A string key"))
                .arg(Arg::new("VALUE").required(true).help("The String value of key"))
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true))
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a give key")
                .arg(Arg::new("KEY").help("A string key").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get", matches)) => {
            let key:String = matches.get_one::<String>("KEY").unwrap().into();

            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found")
            }
        }
        Some(("set", matches)) => {
            let key = matches.get_one::<String>("KEY").unwrap().into();
            let value = matches.get_one::<String>("VALUE").unwrap().into();
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key, value)?;
        }
        Some(("rm", matches)) => {
            let key = matches.get_one::<String>("KEY").unwrap().into();
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key) {
                Ok(()) => {},
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}