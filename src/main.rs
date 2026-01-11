mod ip_addr;
mod parse_address;
mod port_scan;

use clap::Parser;
use parse_address:: {
    parse_arguments
};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    host: String,

    #[arg(long)]
    port: String,
}

fn main() {
    let args = Args::parse();
    let (hosts, ports) = parse_arguments(&args);
    dbg!(&hosts);
    dbg!(&ports);

}
