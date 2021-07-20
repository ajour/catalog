use core::{
    backend::{Backend, Source::*},
    error::Error,
};
use futures::{executor::block_on, try_join};
use std::fs::File;
use std::io::Write;
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
            let (tukui, wowi, curse, townlong_yak, wowup_hub) = try_join!(
                Tukui.get_addons(),
                WowI.get_addons(),
                Curse.get_addons(),
                TownlongYak.get_addons(),
                WowUpHub.get_addons()
            )?;
            // Combine all addons.
            let concatenated = [
                &tukui[..],
                &wowi[..],
                &curse[..],
                &townlong_yak[..],
                &wowup_hub[..],
            ]
            .concat();
            // Serialize.
            let json = serde_json::to_string(&concatenated)?;
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
