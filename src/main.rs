
use std::{thread, time};
use clap::{AppSettings, Clap};
use rand::Rng;

#[derive(Clap)]
#[clap(version = "0.2", author = "ajs <andy@slivers.net>",
       about="Sleeps for a random amount of time (default 10 seconds) \
              before returning.  Might be useful for staggering cron \
              jobs or other things.")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(short, long, about="Prints how long we will sleep for" ) ]
    verbose: bool,
    #[clap(long, default_value="0", about="Sleep for at least many seconds in decimals (eg 1.5)")]
    min: f64,
    #[clap(long, default_value="10", about="Sleep at most many seconds (in decimals)")]
    max: f64,
}

fn main() {
    let mut rng = rand::thread_rng();
   
    let opts: Args = Args::parse();
    
    let rnd_int = rng.gen_range(opts.min..opts.max);
    
    let millis = time::Duration::from_millis((rnd_int * 1000.0) as u64);
    if opts.verbose {
        println!("Sleeping for {:?}", millis);
    }
    thread::sleep(millis);
}
