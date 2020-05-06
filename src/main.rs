use clap::Clap;
use colored::*;
use tabular::{Row, Table};

mod fusion_core;
use fusion_core::{term, Network};

/// Fusion is simple wlan management tool with unix-like commands syntax.
///
#[derive(Clap)]
#[clap(version = "0.4.0", author = "V1oL3nc")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Lists wlan networks nearby
    #[clap(name = "ls", version = "0.1.0")]
    List(List),
    /// Disconnects current connection
    #[clap(name = "kill", version = "0.4.0")]
    Kill(Kill),
}

#[derive(Clap)]
struct Kill {
}

#[derive(Clap)]
struct List {
    /// Uses a long listing format
    #[clap(short = "l", version = "0.2.0")]
    as_long_list: bool,
    /// Prints with human readable data
    #[clap(short = "h", version = "0.3.0")]
    as_human_readable: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List(options) => {
            let list_as_csv_string: String = String::from_utf8({
                std::process::Command::new("sh")
                    .args(&["-c", "nmcli --fields bssid,ssid,chan,freq,signal,security dev wifi|awk -F '[[:space:]][[:space:]]+' {'print $1\"\t\"$2\"\t\"$3\"\t\"$4\"\t\"$5\"\t\"$6'}"])
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
            if options.as_long_list {
                println!("total {}", networks.len());
                let mut table = Table::new("{:<} {:>} {:<} {:>} {:<}");
                for network in networks {
                    table.add_row(
                        Row::new()
                            .with_cell(network.bssid.clone())
                            .with_cell(if !options.as_human_readable {
                                network.channel.clone().to_string()
                            } else {
                                format!("{} MHz", network.frequency.clone())
                            })
                            .with_cell(network.security.clone())
                            .with_cell(network.signal.clone())
                            .with_cell(if network.is_secured() {
                                network.ssid.clone().white()
                            } else {
                                network.ssid.clone().green()
                            }),
                    );
                }
                print!("{}", table);
            } else {
                let names: Vec<ColoredString> = networks
                    .iter()
                    .map(|network| {
                        if network.is_secured() {
                            network.ssid.clone().white()
                        } else {
                            network.ssid.clone().green()
                        }
                    })
                    .collect();
                print!("{}", term::make_vec_printable(names));
            }
        }
        SubCommand::Kill(_) => {
            std::process::Command::new("sh")
                .args(&["-c", "nmcli con down id \"`nmcli connection show|grep wifi|grep -v ' -- '|awk -F '[[:space:]][[:space:]]+' {'print $1'}`\""])
                .output()
                .expect("Failed executing nmcli");
        }
    }
}   
