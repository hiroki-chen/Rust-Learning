pub mod grep_utils;

#[cfg(test)]
mod tests {
  use std::io::BufRead;
  use crate::grep_utils;

  #[test]
  pub fn test_truncate_string() {
    let str1 = "-----";
    let str2 = "--ok";
    let str3 = "-";
    let str4 = "";
    let str5 = "random";

    assert!(grep_utils::truncate_string(str1).is_err());
    assert_eq!("ok", grep_utils::truncate_string(str2).unwrap().as_str());
    assert!(grep_utils::truncate_string(str3).is_err());
    assert!(grep_utils::truncate_string(str4).is_err());
    assert_eq!(str5, grep_utils::truncate_string(str5).unwrap().as_str());
  }

  #[test]
  pub fn test_read_file() {
    dbg!("We are at directory: {}", std::env::current_dir().unwrap().to_str().unwrap());

    let filename = "../README.md";
    assert!(grep_utils::read_file(&filename.to_string()).is_ok());
  }

  #[test]
  #[should_panic(expected = "File does not exist!")]
  pub fn test_read_file_panic() {
    let filename = "./";
    grep_utils::read_file(&filename.to_string()).expect("File does not exist!");
  }

  #[test]
  pub fn test_search_file() {
    // Test one: No environment variable is set.
    std::env::remove_var("CASE_SENSITIVE");
    let filename = "../README.md";
    let mut total_line: usize = 0;
    std::io::BufReader::new(
      std::fs::File::open(filename)
        .unwrap())
      .lines()
      .into_iter()
      .for_each(|_| total_line += 1 );

    let contents = grep_utils::read_file(&filename.to_string()).unwrap();
    let query1 = "Rust";
    let query2 = "rUST";
    let query3 = "";
    let query4 = "^.*$";    // wildcard match.
    let query5 = ".*is.*";  // all lines containing 'is'
    assert_eq!(grep_utils::search_file(&contents, &query1.to_string()).len(), 1);
    assert_eq!(grep_utils::search_file(&contents, &query2.to_string()).len(), 0);
    assert_eq!(grep_utils::search_file(&contents, &query3.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query4.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query5.to_string()).len(), 2);

    // Test two: Environment variable is set to false.
    std::env::set_var("CASE_SENSITIVE", "false");
    assert_eq!(grep_utils::search_file(&contents, &query1.to_string()).len(), 1);
    assert_eq!(grep_utils::search_file(&contents, &query2.to_string()).len(), 1);
    assert_eq!(grep_utils::search_file(&contents, &query3.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query4.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query5.to_string()).len(), 2);

    // Test three: Environment variable is set to true.
    std::env::set_var("CASE_SENSITIVE", "true");
    assert_eq!(grep_utils::search_file(&contents, &query1.to_string()).len(), 1);
    assert_eq!(grep_utils::search_file(&contents, &query2.to_string()).len(), 0);
    assert_eq!(grep_utils::search_file(&contents, &query3.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query4.to_string()).len(), total_line);
    assert_eq!(grep_utils::search_file(&contents, &query5.to_string()).len(), 2);
  }
}