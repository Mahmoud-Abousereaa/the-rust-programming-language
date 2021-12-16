#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

// Using third argument
// impl Config {
//   pub fn new(args: &[String]) -> Result<Config, String> {
//     if args.len() < 3 {
//       return Err(String::from("not enough arguments"));
//     }

//     let query = args[1].to_owned();
//     let filename = args[2].to_owned();
//     let mut case_sensitive = false;
//     if args.len() > 3 {
//       let case_sensitive_option = args[3].to_owned().to_uppercase();

//       if case_sensitive_option.eq("YES") {
//         case_sensitive = true;
//       }
//     }

//     Ok(Config {query, filename, case_sensitive})
//   }
// }

// Using environment variable
impl Config {
  pub fn new(args: &[String]) -> Result<Config, String> {
    if args.len() < 3 {
      return Err(String::from("not enough arguments"));
    }

    let query = args[1].to_owned();
    let filename = args[2].to_owned();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {query, filename, case_sensitive})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  #[allow(unused_variables)]
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }


  Ok(())
}

#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

#[allow(unused_variables)]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_uppercase().contains(&query.to_uppercase()) {
      results.push(line);
    }
  }

  results
}