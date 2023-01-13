extern crate numeronym;

use std::env;
use numeronym::numeronym;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} word", args[0]);
    } else {
        let word = &args[1];
        println!("{}", numeronym(word));
    }
}

