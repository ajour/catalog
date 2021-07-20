use core::{
    backend::{
        curse::Curse, townlong_yak::TownlongYak, tukui::Tukui, wowinterface::WoWInterface, Backend,
    },
    error::Error,
};
use futures::executor::block_on;
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
            let tukui_addons = Tukui {}.get_addons().await?;
            let wowi_addons = WoWInterface {}.get_addons().await?;
            let curse_addons = Curse {}.get_addons().await?;
            let tly_addons = TownlongYak {}.get_addons().await?;
            // Combine all addons.
            let concatenated = [
                &tukui_addons[..],
                &wowi_addons[..],
                &curse_addons[..],
                &tly_addons[..],
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
