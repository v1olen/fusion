use clap::Clap;
use colored::*;
use tabular::{Row, Table};

mod fusion_core;
use fusion_core::{Network, NetworkSecurity, Security};

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
            let (term_width, _) = term_size::dimensions().unwrap();
            let dspace_separated_names = names.join("  ");

            if dspace_separated_names.len() < term_width {
                println!("{}", dspace_separated_names);
            } else {
                let largest_size = names.iter().fold(0, |acc, element| {
                    if element.len() > acc {
                        element.len()
                    } else {
                        acc
                    }
                });
                let columns = (term_width / (largest_size + 2)) as u8;
                let table_structure = (0..columns).map(|_| "{:<}  ").collect::<String>();
                let mut table = Table::new(table_structure.as_str());
                let rows = (names.len() as f32 / columns as f32).ceil() as u8;
                for row in 0..rows {
                    let mut table_row = Row::new();
                    for cell in 0..columns {
                        let index: usize = cell as usize + row as usize * columns as usize;
                        table_row = if index + 1 >= names.len() {
                            table_row.with_cell("")
                        } else {
                            table_row.with_cell({
                                match networks[index].security {
                                    NetworkSecurity(Security::None, Security::None) => {
                                        names[index].clone().green()
                                    }
                                    _ => names[index].clone().red(),
                                }
                            })
                        };
                    }
                    table.add_row(table_row);
                }
                print!("{}", table);
            }
        }
    }
}
