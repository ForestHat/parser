use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::process::Command;
use std::thread;


#[tokio::main]
async fn main() {
    loop {
        clear_database();

        let resp = reqwest::get("https://habr.com/ru/news/").await.expect("Error while parsing site");
        let doc = Document::from(resp.text().await.expect("Error get text!").as_str());

        let mut news_vector: Vec<String> = Vec::new();
        let mut url_vector: Vec<String> = Vec::new();
        
        let mut size: usize = 0;

        for node in doc.find(Class("tm-articles-list__item")) {
            let news: String = node.find(Class("tm-title__link").descendant(Name("span"))).next().expect("Error while parsing site").text();
            news_vector.push(news);

            let url: String = node.find(Class("tm-title__link")).next()
                .expect("Error while parsing site").attr("href").expect("Error while parsing site").to_string();

            url_vector.push(String::from("https://habr.com") + &url);

            size += 1;
        }

        let mut handles = Vec::new();

        for i in 0..size {
            let news: String = news_vector.get(i).expect("Error get").to_string();
            let url: String = url_vector.get(i).expect("Error get").to_string();

            let handle = thread::spawn(move || {
                add_to_database(i, news, url);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Error handle");
        }

        std::thread::sleep(std::time::Duration::from_secs(600));
    }
}

fn add_to_database(index: usize, title: String, url: String) {
    let output = Command::new(String::from("python3")).args(&["main.py", &title]).output().expect("Can't run ai");
    let out = String::from_utf8_lossy(&output.stdout);
    
    Command::new("./main").args(&["-index", &index.to_string(), "-title", &title, "-url", &url, "-theme", &out.replace("\n", "")]).output().expect("Can't add to database");
}

fn clear_database() {
    Command::new("./main").args(&["-clear"]).output().expect("Can't clear the database!");
}
