extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Getting a page from the Internet
    let resp = reqwest::get("https://habr.com/ru/news/").await.unwrap();
    let document = Document::from(resp.text().await.unwrap().as_str());

    // Extract the news
    for node in document.find(Class("tm-articles-list__item")) {
        // Get the text of one news from <span> block
        let news = node.find(Class("tm-title__link").descendant(Name("span")))
            .next()
            .unwrap()
            .text();

        println!("{}", news);
        
        // Get the URL address from block of html
        let url = node.find(Class("tm-title__link")).next().unwrap();

        println!("https://habr.com{}\n", url.attr("href").unwrap());
    }

    Ok(())
}