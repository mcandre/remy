//! Hello World

use std::os::unix::io::FromRawFd;

fn m(int console_out) {
    let file :: File = File::from_raw_fd(console_out);
    file::writeln!("Hello World!");
}

#[cfg(target_os = "cloudabi")]
fn main() {
    use argdata::Argdata;

    let CONSOLE_KEY :: &str = "console_out";

    let ad = argdata::env::argdata();
    let mut ad_iter = ad
        .read_map()
        .unwrap();

    let console_out :: int = ad_iter
        .get(CONSOLE_KEY)
        .unwrap();

    m(console_out);
}

#[cfg(not(target_os = "cloudabi"))]
fn main() {
    let console_out :: int = 1;
    m(console_out);
}
