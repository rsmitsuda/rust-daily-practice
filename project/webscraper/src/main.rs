extern crate reqwest;
extern crate select;
extern crate scraper;
extern crate tokio;

use scraper::{Html, Selector};
// use select::predicate::{Predicate, Attr, Class, Name};
use select::predicate::Name;
use select::document::Document;
// use std::fs::File;

async fn get_website(url: &str) -> Result<String, reqwest::Error> {
    let ret = reqwest::get(url).await?.text().await?;
    let ret_as_bytes = ret.as_bytes();

    // print!("UTFL {:?}\n", String::from_utf8(ret_as_bytes.to_vec()));

    let ret_str = ret.to_string();
    // file_str.write_all(ret.unwrap());

    let response = reqwest::get(url).await?;

	let document = Document::from_read(ret_as_bytes);
    let doc_links = document.unwrap()
    	.find(Name("a"))
    	.filter_map(|links| links.attr("href"))
    	.for_each(|x| println!("{}", x));

    let temp_document = Html::parse_document(&ret);
    let champions = Selector::parse(".champion-index-table__name").unwrap();

    for champion in temp_document.select(&champions) {
    	let champ_text = champion.text().collect::<Vec<_>>();
    	println!("{:?}", champ_text);
    }

    return Ok(ret);
}

#[tokio::main]
async fn main() {
    let url = "https://www.rust-lang.org";
    let league_url = "https://na.op.gg/champion/statistics";

    let response = get_website(&league_url).await;

    // print!("response: {:?}\n", response);

}
