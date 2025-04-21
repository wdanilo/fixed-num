/// This tests compilation on the nightly toolchain.

use fixed_num::*;

fn main() {
    println!("Nightly channel: {}", Dec19x19!(1.0));
}
