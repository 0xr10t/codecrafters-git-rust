use flate2::bufread::ZlibDecoder;
use std::{fs, io::prelude::*};

pub fn cat_file(args: &[String]) {
    if args[0] == "-p"{
        let hash = args[1].clone();
        pretty_print(hash);
    }
}

pub fn pretty_print(hash: String){

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