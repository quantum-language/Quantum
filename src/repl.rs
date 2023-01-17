use std::io;
use std::io::prelude::*;
use std::fs::File;

fn get_version_from_file() -> String {
    let mut file: File = File::open("VERSION").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_version_from_github() -> String {
    let mut file: File = File::open("https://raw.githubusercontent.com/quantum-language/Quantum/master/VERSION").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_version() -> String {
    let version = get_version_from_file();
    let github_version = get_version_from_github();
    if version == github_version {
        return version;
    } else {
        return format!("{} (outdated)", version);
    }
}

fn ask_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

pub fn repl() {
    println!("Quantum Programming Language v{}", get_version());

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let input = ask_user_input();
        
        println!("{}", input);
    }
}