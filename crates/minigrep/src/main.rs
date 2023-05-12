use std::{env, fs};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();



    let config = Config::new(&args);
    println!("{}, {}", config.query, config.filename);

    let content = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("{}", content);

}
