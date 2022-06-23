use std::env;
use std::collections;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::str::FromStr;

use regex::Regex;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor, ColorChoice};

pub fn run<'a>() -> Result<(), Box<dyn Error>>{
  let args = parse_command_line().expect("Parse error!");
  let filename =
    args.get("filename")
      .expect("Please specify the filename by --filename=[name]")
      .as_ref()
      .expect("Filename is empty!");

  let query = match args.get("query") {
    Some(val) => val.as_ref().unwrap().clone(),
    None => String::from(""),
  };

  let mut stderr = StandardStream::stderr(ColorChoice::Always);
  dbg!(colorized_log(&Some(Color::Cyan), &mut stderr, &format!("The filename given is {}.", filename))?);
  println!("{:?}", search_file(&read_file(&filename)?, &query));

  Ok(())
}

/// Converts the string slice into String type and also truncates the dash.
pub fn truncate_string(key: &str) -> Result<String, Box<dyn Error>> {
  let mut index: usize = 0;

  for ch in key.chars() {
    if ch == '-' {
      index += 1;
    } else {
      break;
    }
  }

  if index >= key.len() {
    Err("The key is empty".into())
  } else {
    Ok(key.chars().skip(index).take(key.len() - index).collect::<String>())
  }
}

/// Reads from the command line and returns a hash map containing the keyword-value pairs, indicating
/// specific values for a given option.
pub fn parse_command_line() -> Result<collections::HashMap<String, Option<String>>, Box<dyn Error>> {
  let args = env::args().collect::<Vec<String>>();

  let mut ans: collections::HashMap<String, Option<String>> = collections::HashMap::new();
  for arg in args.into_iter() {
    let key_value: Vec<&str> = arg.split('=').collect::<Vec<&str>>();

    let value: Option<String> =
      if key_value.len() < 2 { None } else { Some(key_value[1].to_string()) };

    // Insert the key value pair into the hash map.
    match truncate_string(key_value[0]) {
      Ok(truncated_string) => {
        ans.entry(truncated_string).or_insert(value);
      }
      Err(e) => return Err(e),
    }
  }

  Ok(ans)
}

/// Prints the colorized text to the standard output stream specified by the user.
/// # Examples
/// ```rust, ignore
///   // Get the handle to the stderr.
///   let mut stderr = StandardStream::stdout(ColorChoice::Always);
///   colorized_log(&Some(Color::Red), &mut stderr, &("Hello world!".to_string())).ok();
/// ```
/// The function will return an error if there is something wrong in print.
pub fn colorized_log(color: &Option<Color>, dest: &mut StandardStream, text: &String)
                     -> Result<(), Box<dyn Error>>
{
  dest.set_color(ColorSpec::new().set_fg(Some(color.unwrap_or(Color::Yellow))))?;
  writeln!(dest, "{}", text)?;
  Ok(())
}

/// Reads the file content given the filename.
/// The content is then split to a vector with linebreaks.
pub fn read_file(filename: &String) -> Result<Vec<String>, Box<dyn Error>> {
  let contents = fs::read_to_string(filename)?;

  dbg!(&contents);
  let mut ans: Vec<String> = Vec::new();
  contents.split('\n').collect::<Vec<&str>>().iter().for_each(|x| {
    ans.push(x.to_string());
  });

  // ans.
  Ok(ans)
}

/// Searches the whole file for the given query string.
/// If the environment variable `CASE_SENSITIVE` is set to false, we will
/// return the whole string into lowercase.
pub fn search_file<'a>(lines: &'a Vec<String>, query: &'a String) -> Vec<&'a String> {
// By default, we want case_sensitive.
  let case_sensitive =
    bool::from_str(env::var("CASE_SENSITIVE")
      .unwrap_or("".to_string()).as_str())
      .unwrap_or(true);
  let query_changed = if case_sensitive { query.clone() } else { query.to_lowercase() };
  // Create a regular expression target.
  let pat = Regex::new(query_changed.as_str()).unwrap();

  let mut ans: Vec<&'a String> = Vec::new();
  for line in lines {
    let line_changed = if case_sensitive { line.clone() } else { line.to_lowercase() };
    if let Some(_) = pat.find(line_changed.as_str()) {
      ans.push(line);
    }
  }

  ans
}
