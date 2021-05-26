use source::tukui::Tukui;
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();
    match opts.command {
        Command::Catalog => {
            let tuk = Tukui {};
            // tuk.get_addons();
            dbg!("doing catalog");
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
