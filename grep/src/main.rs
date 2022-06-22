mod utils;

use termcolor::{StandardStream, ColorChoice};

fn main() {
  let args = utils::parse_command_line().expect("Parse error!");
  let filename =
    args.get("filename")
      .expect("Please specify the filename by --filename=[name]")
      .as_ref().expect("Filename is empty!");

  let mut stdout = StandardStream::stdout(ColorChoice::Always);

  utils::colorized_log(None, &mut stdout, &format!("The filename given is {}.", filename)).expect("OK");
  println!("{:?}", utils::read_file(&filename).expect("The file does not exist!"));
}
