use utils::grep_utils;
use termcolor::{StandardStream, ColorChoice};

fn main() {
  let args = grep_utils::parse_command_line().expect("Parse error!");
  let filename =
    args.get("filename")
      .expect("Please specify the filename by --filename=[name]")
      .as_ref().expect("Filename is empty!");

  let mut stdout = StandardStream::stdout(ColorChoice::Always);

  grep_utils::colorized_log(None, &mut stdout, &format!("The filename given is {}.", filename)).expect("OK");
  let lines = grep_utils::read_file(&filename).expect("The file does not exist!");
  grep_utils::colorized_log(None, &mut stdout, &format!("The content given is {:?}.", lines)).expect("OK");

  println!("Final result : {:?}", grep_utils::search_file(&lines, &"syStem".to_string()));
}
