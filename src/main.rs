use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use serde_json::Value;

fn main() -> serde_json::Result<()> {
    let mut file = File::open("/PATH_TO/keywords.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let keywords: Vec<Value> = serde_json::from_str(contents.as_str())?;

    let mut file = File::open("/PATH_TO/news.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let news: Vec<Value> = serde_json::from_str(contents.as_str())?;

    let start = SystemTime::now();
    let mut found = vec![];

    for news_item in &news {
        let mut stack = vec![];
        for keyword in &keywords {
            let keyword_title = keyword["title"].as_str().unwrap();

            if let Some(_) = news_item["title"].as_str().unwrap().find(keyword_title) {
                stack.push(keyword_title);
            }
            if let Some(_) = news_item["lead"].as_str().unwrap_or("").find(keyword_title) {
                stack.push(keyword_title);
            }
        }
        if !stack.is_empty() {
            found.push((news_item["id"].as_i64().unwrap(), stack));
        }
    }

    println!("Elapsed time (Seconds): {}", start.elapsed().unwrap().as_secs());

    // Uncomment following to see founded ids
    // for item in found {
    //     println!("{}", item.0)
    // }

    Ok(())
}
