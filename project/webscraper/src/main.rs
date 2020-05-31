extern crate reqwest;
extern crate select;
extern crate scraper;
extern crate tokio;

use scraper::{Html, Selector};
use select::predicate::Name;
use select::predicate::Class;
use select::document::Document;

async fn get_website(url: &str) -> Result<String, reqwest::Error> {
	println!("testing 1");

    let ret = reqwest::get(url).await?.text().await?;
	println!("testing 1.1");

    let ret_as_bytes = ret.as_bytes();
	println!("testing 1.2");

    let response = reqwest::get(url).await?;
	println!("testing 2");

    //get op.gg page
	let document = Document::from_read(ret_as_bytes).unwrap();
	println!("testing 3");

    // let temp_document = Html::parse_document(&ret);
    // let champions = Selector::parse(".champion-index-table__name").unwrap();

    //printing out champions [test]
    // for champion in temp_document.select(&champions) {
    // 	let champ_text = champion.text().collect::<Vec<_>>();
    // 	println!("{:?}", champ_text);
    // }

    println!("{:?}", document);

    let mut node_count = 0;
    for node in document.find(Class("champion-index-table__cell--champion")) {

    	// let additional_url = node.find(Name("a"))
    	// 	.filter_map(|links| links.attr("href"))
    	// 	.for_each(|x| println!("{}", x));


    	//is there a way to flatten this into a string?
    	let additional_url = node.find(Name("a"))
    		.filter_map(|links| links.attr("href"))
    		.collect::<Vec<_>>();

		// println!("{:?}", additional_url.first().unwrap());

		let string_to_apped = additional_url.first().unwrap().to_string();
		// let individual_champ_url = url.to_string().push_str(&string_to_apped);
		let individual_champ_url = format!("{}{}", url, string_to_apped);

		// println!("{:?}", url.to_string());
		println!("{:?}", individual_champ_url);

		//fetch the individual champ page
	    // let champ_page = reqwest::get(&individual_champ_url).await?.text().await?;
	    // let page_as_bytes = champ_page.as_bytes();
	    // let champ_doc = Document::from_read(page_as_bytes).unwrap();
	    // let best_items = champ_doc.find(Class(""))

	    // println!("{:?}", champ_response);

	    break;


    }

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
