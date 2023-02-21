use ansi_term::Color::{Green, Red, Yellow};
use august;

pub fn canary() {
    let surfshark_banner = r#"   ___                       __           _                        _
  / __|   _  _      _ _     / _|   ___   | |_     __ _      _ _   | |__
  \__ \  | +| |    | '_|   |  _|  (_-<   | ' \   / _` |    | '_|  | / /
  |___/   \_,_|   _|_|_   _|_|_   /__/_  |_||_|  \__,_|   _|_|_   |_\_\
_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' "#;
    println!("{}", surfshark_banner);
    println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
    println!("\n");

    let url = "https://surfshark.com/warrant-canary";
    let response = reqwest::blocking::get(url).unwrap();

    if response.status().is_success() {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\n",
            response.status(),
            Green.bold().paint("CANARY OPERATIONAL")
        );
    } else if response.status().is_server_error() {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\nVISIT {} FOR FURTHER CONFIRMATION",
            response.status(),
            Red.bold().paint("CANARY DOWN"),
            url
        );
    } else {
        println!(
            "HTTP STATUS CODE:  {:?}\n{}\nVISIT {} FOR FURTHER CONFIRMATION",
            response.status(),
            Yellow.bold().paint("CANARY UNKNOWN"),
            url
        );
    }

    let page = scraper::Html::parse_document(&response.text().unwrap());
    let div = scraper::Selector::parse("div.styles_col12__Y_5s1").unwrap();
    let canary = page.select(&div).next().unwrap();

    println!("🐤  {}", august::convert(&canary.inner_html(), 80));
    println!("\n");
    println!("🐤  {}", Yellow.bold().paint(url));
}
