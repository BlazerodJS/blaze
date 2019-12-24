mod isolate;
mod v8;

fn main() {
  println!("Using V8 {}", v8::version());

  let mut iso = isolate::Isolate::new();
  iso.execute("test.js", "Blazerod.log('Hello from JS ✨'); Blazerod.call(1);");
}
