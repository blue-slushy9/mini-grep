use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Args: query, filepath. Not enough args");
        }
        let query = &args[1].clone();
        let filepath = &args[2].clone();
        let ignore_case = env::var("MG_IGNORE_CASE").is_ok();

        Ok(Config {
            query: query.to_string(),
            filepath: filepath.to_string(),
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    if config.ignore_case {
        let results = search_case_insensitive(&config.query, &contents);
        println!("{:?}", results);
    } else {
        let results = search(&config.query, &contents);
        println!("{:?}", results);
    }

    // println!("first arg: {}", &config.query);
    // println!("second arg: {}", &config.filepath);
    // println!("poem\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line)
        }
        // println!("line is: {line}")
    }
    return matches;
}

pub fn search_case_insensitive<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matches.push(line)
        }
        // println!("line is: {line}")
    }
    return matches;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
