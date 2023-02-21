use clap::Parser;

pub mod nordvpn;
pub mod protonvpn;
pub mod surfshark;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    provider: String,
}

fn usage() {
    println!("vpn providers offering canaries:");
    println!("    surfshark");
    println!("    nordvpn");
    println!("    protonvpn");
}

fn main() {
    let args = Args::parse();
    match args.provider.as_str() {
        "surfshark" => surfshark::canary(),
        "nordvpn" => nordvpn::canary(),
        "protonvpn" => protonvpn::canary(),
        _ => usage(),
    }
}
