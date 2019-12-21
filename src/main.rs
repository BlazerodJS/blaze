mod isolate;
mod v8;

fn main() {
  println!("Using V8 {}", v8::version());

  v8::init();

  let mut iso = isolate::Isolate::new();
  let ret = iso.execute("test.js", "V8Engine.log('Hello from JS ✨')");
  println!("Return code {}", ret);
}
