fn main() {
    let s = String::from("hello");
    take_ownership(s);

    println!("{}, world!", s);
}
fn take_ownership(s: String) { }
