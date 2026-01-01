#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs::{self, File};
// #[allow(unused_imports)]
// use clap::{Parser, Subcommand};
// #[allow(unused_imports)]
// use anyhow::Context;
// #[allow(unused_imports)]
// use flate2::read::ZlibDecoder;
// #[allow(unused_imports)]
// use std::ffi::CStr;
// #[allow(unused_imports)]
// use std::io;
// #[allow(unused_imports)]
// use std::io::BufReader;
#[allow(unused_imports)]
use std::io::prelude::*;
use flate2::bufread::ZlibDecoder;
#[allow(unused_imports)]
use flate2::read::GzDecoder;



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // pub struct args{
    //     command:Commands,

    // }

    // #[derive(Debug,Subcommand)]
    // pub enum  Commands{
    //     Init,
    //     CatFile

    // }

    // TODO: Uncomment the code below to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    }
    else if args[1] == "cat-file" {
        if args[2] == "-p"{
            let hash = args[3].as_str();
            let folder_name = &hash[..2].to_string();
            let file_name  = &hash[2..].to_string();
            let path = format!(".git/objects/{folder_name}/{file_name}");
            let mut object = fs::File::open(path).unwrap();
            let mut content: Vec<u8> = vec![];
            let mut extracted_content = String::new();
            object.read_to_end(&mut content).unwrap();
            let mut d = ZlibDecoder::new(content.as_slice());
            d.read_to_string(&mut extracted_content).unwrap();
            if let Some(null_pos) = extracted_content.find('\0') {
                let content_only = &extracted_content[null_pos + 1..]; 
                print!("{}", content_only);
            }
        } 
    }  else {
        println!("unknown command: {}", args[1])
    }
}

