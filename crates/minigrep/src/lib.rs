
pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config{
    pub fn new(args: &[String])->Config{
        let query = args[1].clone();
        let filename = "crates/minigrep/".to_owned()  + &args[2];
        Config {query, filename}
    }
}