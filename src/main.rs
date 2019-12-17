mod v8;

fn main() {
  println!("Using V8 {}", v8::version());

  v8::init();

  let iso = v8::new_isolate();
  let ret = v8::execute(iso, "test.js", "V8Engine.log('hello from JS')");
  println!("Return code {}", ret);
}
