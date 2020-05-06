use clap::Clap;
use colored::*;

mod fusion_core;
use fusion_core::{Network, NetworkSecurity, Security, term};

/// Simple wlan management tool with gnu-like syntax
///
#[derive(Clap)]
#[clap(version = "1.0", author = "V1oL3nc")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "ls", version = "1.0")]
    List(List),
}

#[derive(Clap)]
struct List {}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List(_) => {
            let list_as_csv_string: String = String::from_utf8({
                std::process::Command::new("sh")
                    .args(&["-c", "nmcli --fields bssid,ssid,chan,signal,security dev wifi|awk -F '[[:space:]][[:space:]]+' {'print $1\"\t\"$2\"\t\"$3\"\t\"$4\"\t\"$5'}"])
                    .output()
                    .expect("Failed executing nmcli")
                    .stdout
            }).expect("nmcli stdout is malformed");

            let mut csv_reader = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_reader(list_as_csv_string.as_bytes());

            let networks: Vec<Network> = csv_reader
                .deserialize()
                .map(|record| Network::from(record.unwrap()))
                .collect();

            let names: Vec<String> = networks
                .iter()
                .map(|network| network.ssid.clone())
                .collect();
            
            print!("{}", term::make_vec_printable(names));
        }
    }
}
