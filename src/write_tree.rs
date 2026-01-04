use std::fs::{self, read_dir};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use sha1 ::{Sha1, Digest};
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;

pub struct TreeEntry{
    pub mode: Mode,
    pub name: String,
    pub sha: String,
}

#[derive(Clone, Copy,Debug)]
pub enum Mode{
    Regular = 100644,
    Executable = 100755,
    Directory = 40000,
}

pub fn write_tree(){
    let root_sha = visit_dirs(Path::new("."));
    let path = "./";
    let mut modes:Vec<String> = Vec::new();
    let mut names:Vec<String> = Vec::new();
    let mut hashes:Vec<String> = Vec::new();
    let entries = read_dir(path);
    for entry in entries.unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let mode = entry.file_type().unwrap();
            let path = entry.path();
            if path.to_str().unwrap().to_string() == ".git"{
                continue;
            }
            if path.is_dir() {
                visit_dirs(&path);
            }
            else{
                hashes.push(write_self(&path.to_str().unwrap().to_string()));
            }
            // modes.push();
        }
        println!("{}", root_sha);
}

fn visit_dirs(dir: &Path) -> String{
    // let mut hashes:Vec<String> = Vec::new();
    let mut entries: Vec<TreeEntry> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name == ".git"{
                continue;
            }
            
            if path.is_dir() {
                let tree_sha = visit_dirs(&path);
                entries.push(TreeEntry{
                    mode: Mode::Directory,
                    name:name,
                    sha:tree_sha,
                });
            } else {
                let blob_hash = write_self(&path.to_str().unwrap().to_string());
                entries.push(TreeEntry {
                mode: Mode::Regular,
                name,
                sha: blob_hash,
            });
            }
        }
    }
    let hash = String::new();
    hash
}

pub fn write_self(filename: &String) -> String {
    //reading input file
    let mut object = File::open(filename).unwrap();
    let mut content:Vec<u8> = vec![];
    object.read_to_end(&mut content).unwrap();
    //creating blob
    let size = content.len();
    let mut blob = Vec::new();
    write!(blob, "blob {}\0", size).unwrap();
    blob.extend_from_slice(&content);
    //hashing blob using sha-1
    let mut hasher = Sha1::new();
    hasher.update(&blob);
    let hash_bytes = hasher.finalize();
    let hash_hex = hex::encode(hash_bytes);
    //creating the object dir
    let (folder_name,file_name) = hash_hex.split_at(2);
    let object_directory = format!(".git/objects/{}",folder_name);
    let object_name = format!("{}/{}",object_directory,file_name);
    fs::create_dir_all(&object_directory).unwrap();
    //writing encoded hashed bytes to object file
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&blob).unwrap();
    let compressed_bytes = e.finish().unwrap();
    fs::write(object_name, compressed_bytes).unwrap();
    hash_hex
}