use source::tukui::Tukui;
use source::Source;
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();
    match opts.command {
        Command::Catalog => {
            let tuk = Tukui {};
            let addons = tuk.get_addons().unwrap();
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
