extern "C" {
  fn HelloWorld();
}

fn main() {
  println!("Hello, world!");
  unsafe {
    HelloWorld();
  }
}
