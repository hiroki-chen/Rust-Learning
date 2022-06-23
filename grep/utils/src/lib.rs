pub mod grep_utils;

#[cfg(test)]
mod tests {
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
}