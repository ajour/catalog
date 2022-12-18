use core::{
    backend::{Backend, Addon, Source::*},
    error::Error,
};
use futures::executor::block_on;
use std::fs::File;
use std::io::Write;
use futures::future::join_all;
use structopt::StructOpt;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let future = handle_opts();
    let _ = block_on(future);
}

async fn handle_opts() -> Result<(), Error> {
    let opts = Opts::from_args();
    match opts.command {
        // Generate a JSON file with all backend sources combined.
        Command::Catalog => {
            let addons: Vec<Addon> = join_all(
                vec![
                    Tukui,
                    WowI,
                    // Curse,
                    Hub
                ].iter().map(|x| x.get_addons())
            ).await
                .into_iter()
                .map(|x| x.unwrap())
                .flatten()
                .collect();
            // Serialize.
            let json = serde_json::to_string(&addons)?;
            // Create catalog file.
            let file_name = format!("catalog-{}.json", VERSION.expect("no version was found"));
            let mut file = File::create(file_name)?;
            // Write to file.
            file.write_all(json.as_bytes())?;
            Ok(())
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Catalog,
}
