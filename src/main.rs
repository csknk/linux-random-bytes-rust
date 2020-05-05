use std::env;
use std::process;
use random_bytes::Config;

//fn main() -> Result<(), &'static str> {
fn main() {
    let args: Vec<String> = env::args().collect();
//    let config = match Config::new(&args) {
//        Ok(config) => config,
//        Err(e) => return Err(e),
//    };
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{}, {}", config.n_bytes, config.format);
    let _ = random_bytes::run(config).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(2);
    });
}
