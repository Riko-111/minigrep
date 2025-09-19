use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() != 3 {
            return Err("Wrong number of arguments provided!")
        }
        Ok(Config { query: args[1].clone(), filepath: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok()})
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filepath.clone())?;

    let results = search(&config.query, &contents, config.ignore_case);

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();

    for line in contents.lines() {
        let to_check = match case_sensitive {
            true => line,
            false => &line.to_lowercase()
        };
        if to_check.contains(query) {
            lines.push(line);
        }
    }

    lines
}

pub fn str_if_contains<'a>(a: &str, b: &str) -> &'a str {
    if a.contains("a") {
        return "zawiera"
    } else if b .contains(b) {
        return "zawiera b"
    }
    "nie zawiera"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "produkt";
        let contents = "\
        Rust:\n\
        bezpiecznie, szybko, produktywnie.\n\
        Wybierz trzy.";
        assert_eq!(vec!["bezpiecznie, szybko, produktywnie."], search(query, contents, false))
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
        Rust:\n\
        bezpiecznie, szybko, produktywnie.\n\
        Wybierz trzy.\n\
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }

    #[test]
    fn case_sensitive() {
        let query = "Kaczka";
        let contents = "\
        Coś co kwacze jak Kaczka,\n\
        Chodzi jak Kaczka,\n\
        Wygląda jak Kaczka\n\
        Jest kaczką.";
        assert_eq!(vec!["Coś co kwacze jak Kaczka,", "Chodzi jak Kaczka,", "Wygląda jak Kaczka"],
                   search(query, contents, true));
    }
}