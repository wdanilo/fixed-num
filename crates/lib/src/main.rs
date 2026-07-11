use fixed_num::traits::*;
use fixed_num::Dec19x19;
use std::str::FromStr;

pub fn main(){
    println!("{:?}", Dec19x19::from_str("100"));
    println!("{:?}", Dec19x19::from_str2("123456.789000e-4"));
}
