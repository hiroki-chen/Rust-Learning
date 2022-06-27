pub enum Nationality {
  US,
  CN,
  JP,
  SG,
}

pub struct Student {
  pub name: String,
  pub age: Option<u8>,
  pub nationality: Option<Nationality>,
}

pub fn change_student(student: &mut Student) -> bool {
  if let Some(age) = student.age {
    println!("{}", age);
    student.age = Some(student.age.unwrap() + 1 as u8);
    true
  } else {
    false
  }
}
