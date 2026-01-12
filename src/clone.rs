// CLI
//  └─ clone command
//      ├─ HTTP client (reqwest)
//      ├─ Git protocol parser
//      ├─ Packfile decoder
//      └─ Object writer (you already have this)

use std::{collections::HashMap, env};

use crate::init::init;


pub async fn clone(args: &[String]){

    //setting the directory structure
    let url = &args[0];
    let main_dir = &args[1];
    env::set_current_dir(main_dir).unwrap();
    init();
    //http get request
    let get_url = format!(
            "{}/info/refs?service=git-upload-pack",
            url.trim_end_matches('/')
        );
    let resp = get_req(&get_url).await;
}

async fn get_req(url: &String) -> HashMap<String,String>{
    //http get requeste
    let resp = reqwest::get(url)
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();
    resp
}