pub mod get; 
pub mod post; 

use crate::methods::get::get::fetch_data as get;
use crate::methods::post::post::post_data as post;

pub async fn run(command: &str) {
    let v: Vec<&str> = command.split_whitespace().collect();

    if v.is_empty() {
        return;
    }

    match v[0] {
        "get" => {
            if v.len() != 2 {
                println!("Usage: get <url>");
                return;
            }
            let result = get(v[1]).await;
            match result {
                Ok(data) => println!("Data: {}", data),
                Err(e) => eprintln!("Fetch Error: {}", e),
            }
        }
        "post" => {
            if v.len() != 3 {
                println!("Usage: post <url> <json_data>");
                return;
            }
            match post(v[1], v[2]).await {
                Ok(resp) => println!("Success: {}", resp),
                Err(e) => eprintln!("Post Error: {}", e),
            }
        }
        _ => println!("Unknown command: {}", v[0]),
    }
}