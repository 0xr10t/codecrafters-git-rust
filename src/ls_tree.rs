use std::{fs, io::Read};
use flate2::bufread::ZlibDecoder;

pub fn ls_tree(args: &[String]){
    if args[0] == "--name-only" {
        let hash = &args[1];
        name_only(hash);
    }
}

pub fn name_only(hash:&String){
    let (dir_name,file_name) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}",dir_name,file_name);
    let mut object = fs::File::open(path).unwrap();
    let mut content: Vec<u8> = vec![];
    let mut extracted_content: Vec<u8> = Vec::new();
    object.read_to_end(&mut content).unwrap();
    let mut decoder = ZlibDecoder::new(content.as_slice());
    decoder.read_to_end(&mut extracted_content).unwrap();
    let header_end = extracted_content.iter().position(|&b| b==b'\0').unwrap();
    let mut cursor = header_end+1;
    while cursor<extracted_content.len(){
        let mode_end = &extracted_content[cursor..].iter().position(|&b| b==b' ').unwrap() + cursor;
        cursor = mode_end+1;
        let name_end = &extracted_content[cursor..].iter().position(|&b| b== b'\0').unwrap()+cursor;
        let name = str::from_utf8(&extracted_content[cursor..name_end]).unwrap();
        cursor = name_end+1;
        println!("{name}");
        cursor += 20;
    }
}