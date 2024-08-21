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

    /// Interval (us) to check job's status
    #[arg(short = 'i', long, default_value_t = DEFAULT_POLLING_INTERVAL)]
    pub polling_interval: u64,

    /// Strategy to manage a job
    #[command(subcommand)]
    pub strategy: Strategy,
}

#[derive(Parser, Debug)]
pub enum Strategy {
    /// Manage the job ignoring job's properties and running state
    Fixed(FixedArgs),

    /// Manage the job by whether job is communicating or not
    CommFocused(FlexibleArgs),

    /// Manage the job by whether job is waiting or not
    WaitFocused(FlexibleArgs),
}

#[derive(Parser, Debug)]
pub struct FixedArgs {
    /// Interval (ms) for which the job is allowed to run
    ///
    /// If timeslice = 0, the job is not switched
    #[arg(short, long)]
    pub timeslice: u64,
}

#[derive(Parser, Debug)]
pub struct FlexibleArgs {
    /// Time (ms) which the job is guaranteed to keep running
    #[arg(short, long)]
    pub timeslice_min: u64,

    /// Time (ms) that the job must be stopped
    #[arg(short = 'T', long)]
    pub timeslice_max: u64,

    /// Time (ms) between job stopping and job starting
    #[arg(short, long)]
    pub sleep_time: u64,
}
