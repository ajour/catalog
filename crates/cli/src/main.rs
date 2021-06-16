use futures::executor::block_on;
use source::curse::Curse;
use source::townlong_yak::TownlongYak;
use source::tukui::Tukui;
use source::wowinterface::WoWInterface;
use source::{Error, Flavor, Source};
use structopt::StructOpt;

fn main() {
    let future = handle_opts();
    let _ = block_on(future);
}

async fn handle_opts() -> Result<(), Error> {
    let opts = Opts::from_args();
    match opts.command {
        Command::Catalog => {
            let curse = Curse {};
            match curse.get_addons().await {
                Ok(addons) => {
                    println!("we have {:?} addons.", addons.len());
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            // let townlong_yak = TownlongYak {};
            // match townlong_yak.get_addons().await {
            //     Ok(addons) => {
            //         println!("{:?}", addons);
            //     }
            //     Err(e) => {
            //         println!("Error: {:?}", e);
            //     }
            // }
            // let wowi = WoWInterface {};
            // match wowi.get_addons().await {
            //     Ok(addons) => {
            //         println!("{:?}", addons);
            //     }
            //     Err(e) => {
            //         println!("Error: {:?}", e);
            //     }
            // }
            // let tuk = Tukui {};
            // match tuk.get_addons(Flavor::Retail).await {
            //     Ok(addons) => {
            //         println!("{:?}", addons);
            //     }
            //     Err(e) => {
            //         println!("Error: {:?}", e);
            //     }
            // }
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
