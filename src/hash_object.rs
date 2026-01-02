use std::fs;
use std::fs::File;
use std::io::Read;
use sha1 ::{Sha1, Digest};
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;


pub fn hash_object(args: &[String]){
    if args[0] == "-w"{
        let filename = &args[1];
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
        println!("{}", hash_hex);
    }
}
