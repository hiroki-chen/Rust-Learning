use std::env;
use std::string;
use std::collections;
use std::error::Error;
use std::fs;
use std::io::Write;

use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// Converts the string slice into String type and also truncates the dash.
fn truncate_string(key: &str) -> Result<String, string::ParseError> {
  let mut index: usize = 0;

  for ch in key.chars() {
    if ch == '-' {
      index += 1;
    } else {
      break;
    }
  }

  Ok(key.chars().skip(index).take(key.len() - index).collect())
}

/// Reads from the command line and returns a hash map containing the keyword-value pairs, indicating
/// specific values for a given option.
pub fn parse_command_line() -> Result<collections::HashMap<String, Option<String>>, string::ParseError> {
  let args = env::args().collect::<Vec<String>>();

  let mut ans: collections::HashMap<String, Option<String>> = collections::HashMap::new();
  for arg in args.into_iter() {
    let key_value: Vec<&str> = arg.split('=').collect();

    let value: Option<String> =
      if key_value.len() < 2 { None } else { Some(key_value[1].to_string()) };

    // Insert the key value pair into the hash map.
    ans.entry(truncate_string(key_value[0])
      .expect("Must be a valid argument!"))
      .or_insert(value);
  }

  Ok(ans)
}

/// Prints the colorized text to the standard output stream specified by the user.
/// # Examples
/// ```
///   let mut stderr = StandardStream::stdout(ColorChoice::Always);
///   colorized_log(None, &mut stdout, &("Hello world!".to_string()))?;
/// ```
/// The function will return an error if there is something wrong in print.
pub fn colorized_log(color: Option<Color>, dest: &mut StandardStream, text: &String)
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

  println!("{:?}", contents);
  let mut ans: Vec<String> = Vec::new();
  contents.split('\n').collect::<Vec<&str>>().iter().for_each(|x| {
    ans.push(x.to_string());
  });

  // ans.
  Ok(ans)
}