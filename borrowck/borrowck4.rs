fn main() {
    let s = String::from("hello");
    borrow(&s);

    println!("{}, world!", s);
}
fn borrow(s: &String) { }
