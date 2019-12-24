use std::env;
use std::fs;

mod isolate;
mod v8;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: blaze <script>");
    return;
  }

  println!("Using V8 {}", v8::version());
  let mut iso = isolate::Isolate::new();

  let filename = &args[1];
  let code = fs::read_to_string(filename).unwrap();
  iso.execute(filename, &code);
}
