use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // .expect("reading the file err");
    let results =if config.case_sensitive {
        search(&config.query,&contents)
    }else {
        search_case_insensitive(&config.query,&contents)
    };
    for line in results {
        println!("{}", line)
    }
    // println!("with text:\n{}", contents);
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive:bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next(){
            Some(arg) => { arg },
            None => return Err("Didn't get a filename string"),
        };
        // let case_sensitive =env::var("CASE_INSENSITIVE").is_err();
        let case_sensitive=false;
        println!("Search for {}", query);
        println!("In file {}", filename);
        Ok(
            Config {
                query,
                filename,
                case_sensitive
            }
        )
    }
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query =query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Duct tape.
Trust me";
        assert_eq!(vec!["Rust:","Trust me"],
                   search_case_insensitive(query, contents))
    }
}