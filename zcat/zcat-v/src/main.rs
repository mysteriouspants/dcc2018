extern crate num;

use num::Integer;

fn main() {
    let upper: u64 = 10000000000;
    let mut acc: u64 = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if n_squared.is_odd() {
            acc += n_squared;
        }
    }

    println!("acc: {}", acc);

    return ();
}
