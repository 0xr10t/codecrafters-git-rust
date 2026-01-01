use std::env;
#[allow(unused_imports)]
use std::fs::{self, File};
use codecrafters_git::{cat_file::cat_file, init::{init}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    match args[1].as_str(){
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        _ => println!("unknown command: {}", args[1]),
    }
}

