mod utilities;

use crate::utilities::bytes_to_hexstring;
use std::io::prelude::*;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub enum Format {
    Hex,
    Int,
}


#[derive(Debug)]
pub struct Config {
    pub n_bytes: u8,
    pub format: Format,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            eprintln!("Please supply 2 arguments: n bytes and format.");
            return Err("Format may be hex or int");
        }
        let n = args[1].trim().clone().parse::<u8>();
        let n_bytes = match n {
            Ok(n) => n,
            Err(_e) => {
                return Err("Cannot parse value to integer.")
            }
        };
        let format_input = args[2].clone().trim().to_string();
        let format = match format_input.as_str() {
            "hex" => Format::Hex,
            "int" => Format::Int,
            _ => Format::Hex
        };
        Ok(Config { n_bytes, format })
    }
}

pub fn run(config: Config) -> io::Result<()> {
    println!("{:?}", config);
    let source = "/dev/urandom";
    let mut f = File::open(source)?;
    let mut buffer = vec![0u8; config.n_bytes as usize];
    f.read(&mut buffer)?;
    println!("buffer = {:?}", buffer);
    println!("{}", bytes_to_hexstring(&buffer, Some("X")));
    Ok(())
}
