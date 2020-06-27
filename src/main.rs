use std::env;
use std::process;
use random_bytes::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let _ = random_bytes::run(config).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(2);
    });
}
