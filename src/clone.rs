// CLI
//  └─ clone command
//      ├─ HTTP client (reqwest)
//      ├─ Git protocol parser
//      ├─ Packfile decoder
//      └─ Object writer (you already have this)

use std::env;

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
    let data = read_pkt_lines(&resp);
}

async fn get_req(url: &String) -> Vec<u8>{
    //http get requeste
    let resp = reqwest::get(url)
        .await.unwrap();
        // .json::<HashMap<String, String>>()
        // .await.unwrap();
    resp.bytes().await.unwrap().to_vec()
}

// response format
// 003f# service=git-upload-pack\n
// 0000
// 003e<sha> refs/heads/main\0<capabilities>\n
// 003d<sha> refs/HEAD\n
// 0000

//convert packet-lines into readable data
fn read_pkt_lines(resp: &[u8]) -> Vec<Vec<u8>>{
    let mut lines = Vec::new();
    let mut cursor = 0;

    while cursor+4 < resp.len(){
        let len = &resp[cursor..cursor+4];
        let len_str = str::from_utf8(len).unwrap();
        let len_usize = usize::from_str_radix(len_str, 16).unwrap();

        cursor+=4;
        if len_usize == 0{
            break;
        }
        let payload_len = len_usize-4;
        let payload = resp[cursor..cursor+payload_len].to_vec();
        lines.push(payload);
    }
    lines
}