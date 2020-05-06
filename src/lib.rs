mod utilities;

use crate::utilities::bytes_to_hexstring;
use std::io::prelude::*;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct Config {
    pub n_bytes: u8,
    pub format: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Please supply 2 arguments: n bytes and format.");
        }
        let n = args[1].trim().clone().parse::<u8>();
        let n_bytes = match n {
            Ok(n) => n,
            Err(_e) => {
                return Err("Cannot parse value to integer.")
            }
        };
        let format = args[2].clone().trim().to_string();
        Ok(Config { n_bytes, format })
    }
}

pub fn run(config: Config) -> io::Result<()> {
    println!("{:?}", config);
//    let mut f = File::open("test")?;
    let mut f = File::open(config.format)?;
    let mut buffer = vec![0u8; config.n_bytes as usize];
    f.read(&mut buffer)?;
    println!("buffer = {:?}", buffer);
    println!("{}", bytes_to_hexstring(&buffer, Some("X")));
    Ok(())
}
