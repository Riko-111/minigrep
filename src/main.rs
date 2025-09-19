use std::env;
use std::process;
use CLIAppBook::Config;
use CLIAppBook::run;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);
    if let Err(e) = run(&config) {
        println!("Application error, {e}");
        process::exit(1);
    }
    dbg!(args);
}