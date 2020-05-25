extern crate reqwest;
extern crate select;
extern crate scraper;
extern crate tokio;

// use scraper::Html;
// use select::predicate::{Predicate, Attr, Class, Name};
// use select::predicate::Name;
use select::document::Document;
use std::fs::File;
use std::convert::From;

async fn get_website(url: &str) -> Result<String, reqwest::Error> {
    let ret = reqwest::get(url).await?.text().await?;
    let ret_as_bytes = ret.as_bytes();

    print!("UTFL {:?}\n", String::from_utf8(ret_as_bytes.to_vec()));

    let ret_str = ret.to_string();
    // file_str.write_all(ret.unwrap());

    let response = reqwest::get(url).await?;


    // let doc = Html::parse_document(&response);

    // for node in response.find(Attr("href", "id")) {
    // 	print!("menu: {:?}\n", node.text());
    // }

    Document::from_read(ret_as_bytes);

    // print!("response: {:?}\n", response1);
    // print!("text: {:?}\n", response2);
    // print!("documetn: {:?}\n", doc);

    // let mut response = match reqwest::get(url).await?.text().await? {
    // 	Ok(good) => return Ok(()),
    // 	Err(e) => return Err(e)
    // };



    return Ok(ret);
}

#[tokio::main]
async fn main() {
    // let url = "https://www.rust-lang.org";
    let url = "https://news.ycombinator.com";


    let response = get_website(&url).await;

    // print!("response: {:?}\n", response);

}
