use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::process::Command;
use std::thread;

#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://habr.com/ru/news/").await.unwrap();
    let doc = Document::from(resp.text().await.unwrap().as_str());

    let mut news_vector: Vec<String> = Vec::new();
    let mut url_vector: Vec<String> = Vec::new();
    
    let mut size: usize = 0;

    for node in doc.find(Class("tm-articles-list__item")) {
        let news: String = node.find(Class("tm-title__link").descendant(Name("span"))).next().unwrap().text();
        news_vector.push(news);

        let url: String = node.find(Class("tm-title__link")).next().unwrap().attr("href").unwrap().to_string();
        url_vector.push(String::from("https://habr.com") + &url);

        size += 1;
    }

    let mut handles = Vec::new();

    for i in 0..size {
        let news: String = news_vector.get(i).unwrap().to_string();
        let url: String = url_vector.get(i).unwrap().to_string();

        let handle = thread::spawn(move || {
            run_cmd(news, url);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn run_cmd(news: String, url: String) {
    let exec: String = String::from("python3");
    let output = Command::new(exec).args(&["main.py", &news]).output().unwrap();
    let out = String::from_utf8_lossy(&output.stdout);

    println!("{}    {}    {}", news, out.replace("\n", ""), url);
}
