use std::env::args;
use std::error::Error;
use std::io::Read;
use std::fs::File;
// use std::time::Instant;
use entrs::*;
use getopts::Options;

fn main() -> Result<(), Box<dyn Error>>{
    let mut opts = Options::new();
    opts.reqopt("f", "file", "Input data", "FILE");
    opts.optopt("o", "output", "Plot output", "FILE");
    opts.optopt("w", "window-size", "Compression window size", "SIZE");
    opts.optopt("s", "step-by", "Window setp size", "SIZE");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(args().skip(1))
        .unwrap_or_else(|e| {
            panic!("Invalid arguments: {}\n{}", e, opts.usage("Usage: entrs -f FILE [OPTIONS]"));
        });

    if matches.opt_present("h") {
        println!("{}", opts.usage("Usage: entrs -f FILE [OPTIONS]"));
        return Ok(());
    }

    let file = matches.opt_str("f").expect("Missing FILE");
    let out = matches.opt_str("o").unwrap_or("plot.png".to_owned());

    let w = matches.opt_str("w")
        .map(|s| s.parse().expect("Invalid w"))
        .unwrap_or( 4096);

    let s = matches.opt_str("s")
        .map(|s| s.parse().expect("Invalid s"))
        .unwrap_or( 64);

    let mut input = File::open(file).unwrap();
    let mut v = Vec::new();

    input.read_to_end(&mut v).expect("IO error");
    let ent = par_compute_ent(&v[..], w, s).expect("Error in computation");

    plot(&ent, &out).expect("Error plotting");

    Ok(())
}
