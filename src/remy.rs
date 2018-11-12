//! CLI application

extern crate getopts;
extern crate remy;

use std::env;
use std::process;

/// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

/// Show version information
fn banner(program : &str) {
    println!("{} {}", program, env!("CARGO_PKG_VERSION"));
}

/// CLI entrypoint.
/// Verbosity is controlled by a VERBOSE environment variable.
fn main() {
    let portconfig = &mut remy::new_portconfig();

    if env::var("VERBOSE").is_ok() {
        portconfig.is_verbose = true;
    }

    let args : Vec<String> = env::args().collect();
    let program : &str = args[0].as_ref();
    let brief = format!("Usage: {} [options] <binary> [<binary> [<binary>...]]", program);

    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");
    opts.optflag("r", "release", "Enable release mode");

    let optresult : Result<getopts::Matches, getopts::Fail> = opts.parse(&args[1..]);

    if optresult.is_err() {
        usage(&brief, &opts);
        process::abort();
    }

    let optmatches : getopts::Matches = optresult.unwrap();

    if optmatches.opt_present("r") {
        portconfig.is_release = true;
    } else if optmatches.opt_present("v") {
        banner(&program);
        process::exit(0);
    } else if optmatches.opt_present("h") {
        usage(&brief, &opts);
        process::exit(0);
    }

    let binaries : Vec<String> = optmatches.free;

    if binaries.is_empty() {
        usage(&brief, &opts);
        process::abort();
    }

    remy::port(portconfig, binaries);
}
