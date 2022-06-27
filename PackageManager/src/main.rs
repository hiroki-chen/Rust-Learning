mod some_module;
mod config;
mod services;

fn main() {
    println!("Hello, world!");

    let mut student: some_module::Student = some_module::Student {
      name: String::from("Fuck"),
      age: None,
      nationality: Some(some_module::Nationality::CN),
    };

    println!("{}", some_module::change_student(&mut student));
    config::print_config();
}
