//! Hello World

use std::io::Write;

#[cfg(target_os = "cloudabi")]
extern crate argdata;

#[cfg(target_os = "cloudabi")]
extern crate cloudabi;

fn m<T: Write>(console_out : &mut T) {
    writeln!(console_out, "Hello World!")
        .unwrap();
}

#[cfg(target_os = "cloudabi")]
fn main() {
    use argdata::Argdata;
    use argdata::ArgdataExt;

    //
    // https://cloudabi.org/write/rust/
    //

    struct Console(cloudabi::fd);

    impl Write for Console {
        fn write(&mut self, buf : &[u8]) -> std::io::Result<usize> {
            unsafe {
                let iovs = [
                    cloudabi::ciovec {
                        buf: (buf.as_ptr() as *const _ as *const (), buf.len()),
                    },
                ];
                let mut n_written = std::mem::uninitialized();
                cloudabi::fd_write(self.0, &iovs, &mut n_written);
            }

            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let console_out_key : &str = "console_out";

    let ad = argdata::env::argdata();
    let mut ad_iter = ad
        .read_map()
        .unwrap()
        .iter_map();

    let mut console_out_fd_res = Err(
        argdata::NotRead::Error(
            argdata::ReadError::InvalidFdNumber(u32::max_value())
        )
    );

    while let Some(Ok((key, value))) = ad_iter.next() {
        if key.read_str().unwrap() == console_out_key {
            console_out_fd_res = value.read_fd();
        }
    }

    let console_out = &mut Console(
        cloudabi::fd(
            console_out_fd_res
                .unwrap()
                .0 as u32
        )
    );
    m(console_out);
}

#[cfg(not(target_os = "cloudabi"))]
fn main() {
    use std::io::stdout;

    let console_out = &mut stdout();
    m(console_out);
}
