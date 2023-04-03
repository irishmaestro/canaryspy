use ansi_term::Color::Yellow;
use clap::{arg, Command};

pub mod nordvpn;
pub mod protonvpn;
pub mod surfshark;

const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn list_canaries() {
    println!("{}", Yellow.bold().paint("nordvpn"));
    println!("{}", Yellow.bold().paint("surfshark"));
    println!("{}", Yellow.bold().paint("protonvpn"));
}

fn cli() -> Command {
    Command::new("canary")
        .about(DESC)
        .version(VERSION)
        .author(AUTHOR)
        .help_template(
            "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
",
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        // PULL
        .subcommand(
            Command::new("pull")
                .short_flag('p')
                .long_flag("pull")
                .about("retrieve canary")
                .arg(arg!(<VPN_PROVIDER>))
                .arg_required_else_help(true),
        )
        // LIST
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("list canaries available")
                .args_conflicts_with_subcommands(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("pull", pull_matches)) => {
            match pull_matches
                .get_one::<String>("VPN_PROVIDER")
                .expect("required")
                .as_str()
            {
                "nordvpn" => nordvpn::canary(),
                "surfshark" => surfshark::canary(),
                "protonvpn" => protonvpn::canary(),
                _ => println!("provider not found. `canary list` to list providers"),
            }
        }
        Some(("list", _list_matches)) => list_canaries(),
        _ => unreachable!(),
    }
}
