use std::rc::Rc;

struct Owner { pub name: String }
struct Node { pub owner: Rc<Owner> }

fn main() {
  let o = Rc::new(Owner { name: "Owner".to_string() });

  let n1 = Node { owner: Rc::clone(&o) }; // cloning the Rc, not the
  let n2 = Node { owner: Rc::clone(&o) }; // O inside of it

  println!("References to o: {}", Rc::strong_count(&o)); // 3

  drop(o); // dispose of local variable o, releasing a reference

  println!("References to o: {}", Rc::strong_count(&n1.owner)); // 2

  // n1 and n2 are destroyed, dropping the refcount of o to
  // zero, so it too is destroyed
}