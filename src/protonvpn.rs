use ansi_term::Color::{Green, Red, Yellow};
use august;

pub fn canary() {
    let protonvpn_banner = r#"    ___                    _                     __   __    ___   _  _   
   | _ \    _ _    ___    | |_     ___    _ _    \ \ / /   | _ \ | \| |  
   |  _/   | '_|  / _ \   |  _|   / _ \  | ' \    \ V /    |  _/ | .` |  
  _|_|_   _|_|_   \___/   _\__|   \___/  |_||_|   _\_/_   _|_|_  |_|\_|  
_| """ |_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_| """"|_| """ |_|"""""| 
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' "#;
    println!("{}", protonvpn_banner);
    println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
    println!("\n");

    let url = "https://protonvpn.com/blog/transparency-report";
    let response = reqwest::blocking::get(url).unwrap();

    if response.status().is_success() {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\n",
            response.status(),
            Green.bold().paint("CANARY OPERATIONAL")
        );
    } else if response.status().is_server_error() {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\nVISIT {} FOR FURTHER CONFIRMATION.",
            response.status(),
            Red.bold().paint("CANARY DOWN"),
            url
        );
    } else {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\nVISIT {} FOR FURTHER CONFIRMATION.",
            response.status(),
            Yellow.bold().paint("CANARY UNKNOWN"),
            url
        );
    }

    let page = scraper::Html::parse_document(&response.text().unwrap());
    let title = scraper::Selector::parse("h1").unwrap();
    let date = scraper::Selector::parse("p.meta").unwrap();
    let entry = scraper::Selector::parse("div.entry").unwrap();

    let canary_title = page.select(&title).next().unwrap();
    let canary_date = page.select(&date).next().unwrap();
    let canary = page.select(&entry).next().unwrap();

    println!("🐤  {}", august::convert(&canary_title.inner_html(), 80));
    println!("\n");
    println!("{}", august::convert(&canary_date.inner_html(), 80));
    println!("\n");
    println!("{}", august::convert(&canary.inner_html(), 80));
    println!("\n");
    println!("🐤  {}", Yellow.bold().paint(url));
}
