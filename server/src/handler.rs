use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use crate::{constant, thread_pool};

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

fn read_and_write(file_name: &str, stream: &mut TcpStream, header: &str) -> Result<(), Box<dyn std::error::Error>> {
  if let Ok(file) = fs::read_to_string(file_name) {
    let response = format!(
      "{}Content-Length:{}\r\n\r\n{}",
      header,
      file.len(),
      file);

    stream.write(response.as_bytes())?;
    stream.flush()?;
  } else {
    panic!("In function {}: Read file failed!", function!());
  }

  Ok(())
}

pub fn run(thread_num: usize, address: &String) -> Result<(), Box<dyn std::error::Error>> {
  let tcp = TcpListener::bind(address)
    .unwrap_or_else(|e| panic!("{}", e));

  // Create the thread pool.
  let thread_pool = thread_pool::ThreadPool::new(thread_num)?;

  // We dispatch each connection stream to each available thread.
  for stream in tcp.incoming().take(2) {
    let mut s = stream.unwrap_or_else(|e| panic!("{}", e));

    println!("Connection established! Peer: {}", s.peer_addr().unwrap().to_string());
    thread_pool.execute(move || {
      handle_connection(&mut s)
        .unwrap_or_else(|e| panic!("{}", e.to_string()));
    })?;
  }

  println!("Shutting down the server...");

  Ok(())
}

pub fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
  let mut buf = [0 as u8; 1024];

  stream.read(&mut buf).unwrap();

  let (file_name, header) = if buf.starts_with(constant::HTTP_HEAD_GET.as_bytes()) {
    (constant::FILE_NAME_INDEX, constant::HTTP_HEADER_OK)
  } else {
    (constant::FILE_NAME_404, constant::HTTP_HEADER_NOT_FOUND)
  };
  read_and_write(file_name, stream, header)?;

  Ok(())
}