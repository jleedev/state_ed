use std::io;
use std::io::BufRead;

mod lib;
use lib::*;

fn main() {
    let stdin = io::stdin();
    let mut ed = Ed::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        ed.send(line.into());
    }
}
