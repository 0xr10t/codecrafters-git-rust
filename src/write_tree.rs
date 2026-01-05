use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use sha1 ::{Sha1, Digest};
use std::io::Write;
use flate2::{Compression};
use flate2::write::ZlibEncoder;
use std::os::unix::fs::PermissionsExt;

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
            } else if path.is_file(){
                let blob_hash = write_self(&path.to_str().unwrap().to_string());
                let metadata = fs::metadata(&path).unwrap();
                let perm_mode = metadata.permissions().mode();
                let mode = if perm_mode & 0o111 != 0 {
                    Mode::Executable
                } else {
                    Mode::Regular
                };
                entries.push(TreeEntry {
                    mode,
                    name,
                    sha: blob_hash,
                });
            }
            }
    }
    entries.sort_by_key(|e| e.name.clone());
    let mut body = Vec::new();

    for entry in entries{
        write!(body,
            "{} {}\0",
            entry.mode as u32,
            entry.name       
        ).unwrap();

        let sha1_hash = hex::decode(entry.sha).unwrap();
        body.extend_from_slice(&sha1_hash);
    }

    let mut tree = Vec::new();
    write!(tree, "tree {}\0", body.len()).unwrap();
    tree.extend_from_slice(&body);
    // write_object(tree);
    let mut hasher = Sha1::new();
    hasher.update(&tree);
    let hash_bytes = hasher.finalize();
    let hash_hex = hex::encode(hash_bytes);
    let (folder_name,file_name) = hash_hex.split_at(2);
    let object_directory = format!(".git/objects/{}",folder_name);
    let object_name = format!("{}/{}",object_directory,file_name);
    fs::create_dir_all(&object_directory).unwrap();
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&tree).unwrap();
    let compressed_bytes = e.finish().unwrap();
    fs::write(object_name, compressed_bytes).unwrap();
    hash_hex
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