extern crate reqwest;
extern crate select;
extern crate scraper;
extern crate tokio;

use scraper::{Html, Selector};
// use select::predicate::{Predicate, Attr, Class, Name};
use select::predicate::Name;
use select::predicate::Class;
use select::document::Document;
// use std::fs::File;

async fn get_website(url: &str) -> Result<String, reqwest::Error> {
	println!("testing 1");

    let ret = reqwest::get(url).await?.text().await?;
	println!("testing 1.1");

    let ret_as_bytes = ret.as_bytes();
	println!("testing 1.2");


    // print!("UTFL {:?}\n", String::from_utf8(ret_as_bytes.to_vec()));

    let response = reqwest::get(url).await?;
	println!("testing 2");


    //get op.gg page
	let document = Document::from_read(ret_as_bytes).unwrap();
	println!("testing 3");


	//get all links on the op.gg homepage
    let doc_links = document.find(Name("a"))
    	.filter_map(|links| links.attr("href"))
    	.for_each(|x| println!("{}", x));
	println!("testing 4");


    let temp_document = Html::parse_document(&ret);
    let champions = Selector::parse(".champion-index-table__name").unwrap();

    //printing out champions [test]
    for champion in temp_document.select(&champions) {
    	let champ_text = champion.text().collect::<Vec<_>>();
    	println!("{:?}", champ_text);
    }

    let mut champion_count = 0;
    for node in document.find(Class("champion-index-table__cell--champion")) {
    	let items = node;
    	println!("{:?}", items);
    	champion_count += 1;
    }
	println!("{:?}", champion_count);



	println!("testing end");
    return Ok(ret);
}

#[tokio::main]
async fn main() {
    let url = "https://www.rust-lang.org";
    let league_url = "https://na.op.gg/champion/statistics";

    get_website(&league_url).await;

    // print!("response: {:?}\n", response);

}
