use ansi_term::Color::{Green, Red, Yellow};
use august;

pub fn canary() {
    let nordvpn_banner = r#"  _  _                       _   __   __    ___   _  _
 | \| |    ___      _ _   __| |  \ \ / /   | _ \ | \| |
 | .` |   / _ \    | '_| / _` |   \ V /    |  _/ | .` |
 |_|\_|   \___/   _|_|_  \__,_|   _\_/_   _|_|_  |_|\_|
_|"""""|_|"""""|_|"""""|_|"""""|_| """"|_| """ |_|"""""|
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' "#;
    println!("{}", nordvpn_banner);
    println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
    println!("\n");

    // let url = "https://nordvpn.com/security-efforts/";
    let url = "https://nordvpn.com/blog/nordvpn-introduces-a-warrant-canary/";
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
    let div = scraper::Selector::parse("div.BlogPost__content").unwrap();
    let canary = page.select(&div).next().unwrap();

    let full = august::convert(&canary.inner_html(), 80);
    let rm_rf_sales_pitch = full.as_str().get(0..1638);
    println!(
        "🐤  {}",
        rm_rf_sales_pitch.unwrap_or(
            "ERROR PULLING CANARY: VISIT THE FOLLOWING URL TO MANUALLY VERIFY CANARY STATUS"
        )
    );

    println!("\n");
    println!("🐤  {}", Yellow.bold().paint(url));
}
