mod ip_addr;
mod parse_address;
mod port_scan;
mod syn;

use clap::{Args, Parser};
use parse_address::{
    parse_arguments,
    ScanInfo
};
use port_scan::port_scan;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(long)]
    host: String,

    #[arg(long)]
    port: String,

    #[arg(long, short, help = "Set timeout", default_value = "100")]
    timeout: Option<u32>,

    #[command(flatten)]
    scan_options: ScanOptions,
}

#[derive(Args)]
#[group(multiple = false, required = false)]
struct ScanOptions {
    #[arg(short = 'C', long = "sC", help = "Connect scan")]
    connect: bool,

    #[arg(short = 'S', long = "sS", help = "Syn scan")]
    syn: bool,

    #[arg(short = 'U', long = "sU", help = "UDP scan")]
    udp: bool
}


fn main() {
    let args = Arguments::parse();
    let mut scan_options: ScanInfo = parse_arguments(&args);
    port_scan(&mut scan_options);
}
