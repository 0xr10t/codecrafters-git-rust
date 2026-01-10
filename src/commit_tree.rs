use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;

pub fn commit_tree(args: &[String]) {
    let tree_sha = &args[0];
    let parent_sha = &args[2];
    let message = &args[4];
    let mut body:Vec<u8> = Vec::new();
    write!(body,
            "tree {}\nparent {}\nauthor Soham <sohamvijaya1000@gmail.com> 1234567890 +0000\ncommitter Soham <sohamvijaya1000@gmail.com> 1234567890 +0000\n\n{}\n",
            tree_sha,
            parent_sha,
            message
        ).unwrap();

    let mut commit_write:Vec<u8> = Vec::new();
    write!(
        commit_write, "commit {}\0",
        body.len()
    ).unwrap();
    commit_write.extend_from_slice(&body);
    //hashing blob using sha-1
    let mut hasher = Sha1::new();
    hasher.update(&commit_write);
    let hash_bytes = hasher.finalize();
    let hash_hex = hex::encode(hash_bytes);
    //creating the object dir
    let (folder_name,file_name) = hash_hex.split_at(2);
    let object_directory = format!(".git/objects/{}",folder_name);
    let object_name = format!("{}/{}",object_directory,file_name);
    fs::create_dir_all(&object_directory).unwrap();
    //writing encoded hashed bytes to object file
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&commit_write).unwrap();
    let compressed_bytes = e.finish().unwrap();
    fs::write(object_name, compressed_bytes).unwrap();
    println!("{}", hash_hex);    
}