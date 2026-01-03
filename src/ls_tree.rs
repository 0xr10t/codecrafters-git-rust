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
    let mut extracted_content = String::new();
    object.read_to_end(&mut content).unwrap();
    let mut decoder = ZlibDecoder::new(content.as_slice());
    decoder.read_to_string(&mut extracted_content).unwrap();
    let null_pos_w = extracted_content.find('\0');
    let (_header, rest) = extracted_content.split_at(null_pos_w.unwrap());
    let iter = rest.split_ascii_whitespace();
    for (i,s) in iter.enumerate(){
        if i%2==0{
            let pos_w = s.find('\0');
            let (name,_hash) = s.split_at(pos_w.unwrap());
            println!("{name}");
        }
    }
}