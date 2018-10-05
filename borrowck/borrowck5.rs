struct MyStruct {
  pub lambda: Box<Fn()> // store a lambda to call later
}
fn main() {
  let s = "hello";
  let speaker = MyStruct {
    // the move keyword transfers ownership of s
    // to the lambda - s is now invalid within main
    lambda: Box::new(move || println!("{}, world", s))
  };
  // call the lambda
  (speaker.lambda)();
}