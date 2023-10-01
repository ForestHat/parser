extern crate reqwest;
extern crate select;

use std::fs::File;
use std::io::Write;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("news.txt").expect("Error! File not created");
    for pages in 25..50 {
        let page: String = "https://habr.com/ru/news/page".to_owned() + &pages.to_string();
        
        // Getting a page from the Internet
        let resp = reqwest::get(&page).await.unwrap();
        let document = Document::from(resp.text().await.unwrap().as_str());
    
        // Extract the news
        for node in document.find(Class("tm-articles-list__item")) {
            // Get the text of one news from <span> block
            let news = node.find(Class("tm-title__link").descendant(Name("span")))
                .next()
                .unwrap()
                .text();
    
            println!("{}", news);
            
            let news_to_file: &str = &(news + "\n");
            file.write_all(news_to_file.as_bytes()).expect("Problem with writing in file!");
            
            // Get the URL address from block of html
            let url = node.find(Class("tm-title__link")).next().unwrap();
            println!("https://habr.com{}\n", url.attr("href").unwrap());
        }
    }
    Ok(())
}
