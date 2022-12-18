use clap::{App, Arg};
use std::error::Error;
use rand::Rng;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    new: bool
}

// #[derive(Debug)]
// pub struct Guess {
//     word: String
// }

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wordlr")
        .version("0.1.0")
        .author("Alex Tran <lbhtran@gmail.com>")
        .about("A CLI wordle clone written in Rust")
        .arg(
            Arg::with_name("new")
            .value_name("NEW")
            .help("New game")
            .takes_value(true)
            )
        .get_matches();
    Ok(Config {
        new: matches.is_present("new")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let words: [&str; 3] = ["bird", "frog", "deer"];
    if config.new {
        let idx = rand::thread_rng().gen_range(0..3);
        println!("The word is: {}", words[idx]);
    }
    Ok(())
}
