// 'use' is similar to 'import' in Python or 'using' in C++, it brings in 
// modules, functions, etc. from libraries;
// 'std::env' brings the env module from the std Rust library into the
// current scope, so that you don't have to write 'std::env' every time;
// 'env' is a module that provides functions for interacting with the
// environment of the current process, e.g. getting or setting env variables
use std::env;
// 'Error' is an essential trait (these define a set of methods that a type 
// can implement) from the 'error' module that represents a generic error type;
// the 'error' module is designed for error handling
use std::error::Error;
// 'fs' allows you to interact with files and directories on the OS,
// e.g. creating, reading, writing
use std::fs;
// 'pub' means the public visibility modifier, so the struct can be accessed 
// from other modules or crates; the default vis mod for Rust is private,
// i.e. it's only accessible within the current module; 
// 'Config' is just the name of the struct
pub struct Config {
    // 'query' is the field, its type is String
    query: String,
    filepath: String,
    // determines whether search should be case-sensitive or not
    ignore_case: bool,
}

// 'impl' defines methods associated with struct Config
impl Config {
    // This line is the function signature; public function 'build', 
    // which takes a slice '&[]' of 'String' as its argument; typically
    // represents CLI arguments, also '&' means reference as in C/C++;
    // '-> Result [...] str>' is the return type; 'Result' is an enum used for
    // error handling, can either be 'Ok(Config)' which is success, or it can
    // be 'Err(&'static str)' which is failure; 
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Verify enough arguments are passed based on struct Config definition;
        // the three arguments are: the program name itself, query, filepath;
        if args.len() < 3 {
            return Err("Args: query, filepath. Not enough arguments.");
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
