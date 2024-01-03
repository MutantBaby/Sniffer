use clap::Parser;
use std::net::IpAddr;

/// To monitoring & capturing all data packets passing through given network
#[derive(Parser, Debug)]
struct Cli {
    /// The Flag for help
    flag: String,
    /// The ip-address to listen on
    ipadder: IpAddr,
    /// Number of threads to use
    threads: u16,
}

impl Cli {}

fn main() {
    let args: Cli = Cli::parse();

    println!("Args {:?}", args);
}
