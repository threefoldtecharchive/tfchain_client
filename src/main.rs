use keyring::AccountKeyring;
use git_version::git_version;
use clap::{App, Arg};

extern crate tokio;

const GIT_VERSION: &str = git_version!(args = ["--tags", "--always", "--dirty=-modified"]);

#[tokio::main]
async fn main() {
    let matches = App::new("tfchaincli")
        .author("ThreeFold Tech, https://github.com/threefoldtech")
        .version(GIT_VERSION)
        .about("A tfchain command line client")
        .arg(Arg::new("websocket").value_name("WEBSOCKET").short('s').long("websocket").default_value("wss://tfchain.dev.grid.tf").help("substrate websocket connection"))
        .subcommand(
            App::new("farms")
            .about("Query the farms.")
            .arg(
                Arg::new("id")
                    .help("get by id")
                    .takes_value(true)
                    .required(true)
            )
        )
        .get_matches();

    let websocket = matches.value_of("websocket").unwrap();

    match matches.subcommand() {
        Some(("farms", farm_matches)) => {
            if farm_matches.is_present("id") {
                let from = AccountKeyring::Alice.pair();
                let client = tfchain_client::Client::new(String::from(websocket), from);
                let id = farm_matches.value_of("id").unwrap();
                match id.parse::<u32>() {
                    Ok(x) => {
                        let farm = client.get_farm_by_id(x).unwrap();
                        println!("farm: {:?}", farm);
                    },
                    Err(_) => {
                        println!("not a number");
                        // Err(e)
                    }
                }
            }

        },
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    };
    
    // if let Err(e) = result {
    //     println!("error: {:?}", e);
    //     std::process::exit(1)
    // }
}