use std::env;
use codecrafters_git::{cat_file::cat_file, clone::clone, commit_tree::commit_tree, hash_object::hash_object, init::init, ls_tree::ls_tree, write_tree::write_tree};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    match args[1].as_str(){
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),
        "ls-tree" => ls_tree(&args[2..]),
        "write-tree" => write_tree(),
        "commit-tree" =>  commit_tree(&args[2..]),
        "clone" => clone(&args[2..]).await,
        _ => println!("unknown command: {}", args[1]),
    }
}

