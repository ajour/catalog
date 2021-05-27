use futures::executor::block_on;
use source::tukui::Tukui;
use source::Source;
use structopt::StructOpt;

fn main() {
    let future = handle_opts();
    block_on(future);
}

async fn handle_opts() {
    let opts = Opts::from_args();
    match opts.command {
        Command::Catalog => {
            let tuk = Tukui {};
            let addons = tuk.get_addons().await.unwrap();
            dbg!("tukui addons: {}", addons);
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
