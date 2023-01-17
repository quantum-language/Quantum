mod vm;
mod repl;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use crate::repl::repl;

fn main() {
    let mut file: File = File::open("examples/Token.quantum").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();

    repl();
}
