mod ip_addr;
mod parse_address;
mod port_scan;

use clap::Parser;
use parse_address:: {
    ScanInfo,
    parse_arguments
};
use port_scan::port_scan;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    host: String,

    #[arg(long)]
    port: String,

    #[arg(long, short, help = "Set timeout", default_value = "100")]
    timeout: Option<u32>,

    #[arg(long = "sS", help = "SYN scan")]
    syn: bool,

    #[arg(long = "sC", help = "Connect scan")]
    connect: bool
}


fn main() {
    let args = Args::parse();
    let mut scan_options: ScanInfo = parse_arguments(&args);
    port_scan(&mut scan_options);

}
