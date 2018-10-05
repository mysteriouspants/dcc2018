extern crate num;

use num::Integer;

fn main() {
  let upper: u64 = 10000000000;
  let sum_of_squared_odd_numbers: u64 =
    (0..).map(|n| n * n)
         .take_while(|&n| n < upper)
         .filter(|n| (*n).is_odd())
         .fold(0, |sum, i| sum + i);
  
  println!("acc: {}", sum_of_squared_odd_numbers);

  return ();
}
