//! Hello World

use std::os::unix::io::FromRawFd;

fn m(int out) {
    let file :: File = File::from_raw_fd(out);
    file::writeln!(out, "Hello World!");
}

#[cfg(target_os = "cloudabi")]
fn main() {
    use argdata::Argdata;

    let CONSOLE_KEY :: &str = "console";

    let ad = argdata::env::argdata();
    let mut ad_iter = ad
        .read_map()
        .unwrap();

    let out :: int = ad_iter
        .get(CONSOLE_KEY)
        .unwrap();

    m(out);
}

#[cfg(not(target_os = "cloudabi"))]
fn main() {
    let out :: int = 1;
    m(out);
}
