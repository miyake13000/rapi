use clap::Parser;
use rapi::*; // import some consts
use simplelog::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Port to bind
    #[arg(short = 'p', long, default_value_t = DEFAULT_RAPICTLD_PORT)]
    pub port: u16,

    /// The list of all rapid's addresses (IP address or domain).
    /// Example: "node1, node2" or "192.168.1.2, 192.168.1.3"
    #[arg(short = 'a', long, required = true, value_delimiter = ',')]
    pub rapid_addrs: Vec<String>,

    /// Port of rapid (All rapid's port must be same)
    #[arg(short = 'P', long, default_value_t = DEFAULT_RAPID_PORT)]
    pub rapid_port: u16,

    /// Debug level (One of [Error, Warn, Info, Debug, Trace, Off])
    #[arg(short = 'd', long, default_value_t = DEFAULT_DLEVEL)]
    pub debug: LevelFilter,
}
