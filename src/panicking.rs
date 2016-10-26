use core::fmt::{self, Write};
use core::result;

use syscall::{write, exit};

pub struct DebugStream;

impl Write for DebugStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Err(_err) = write(2, s.as_bytes()) {
            result::Result::Err(fmt::Error)
        } else {
            result::Result::Ok(())
        }
    }
}

#[lang="panic_fmt"]
#[allow(unused_must_use)]
pub extern "C" fn rust_begin_panic(fmt: ::core::fmt::Arguments, file: &str, line: u32) -> ! {
    let mut stream = DebugStream;
    stream.write_fmt(format_args!("Panic in {}:{}: {}\n", file, line, fmt));

    loop {
        let _ = exit(128);
    }
}

#[inline(never)]
#[cold]
pub fn begin_panic(string: &'static str, file_line: &(&'static str, u32)) -> ! {
    let &(file, line) = file_line;

    rust_begin_panic(format_args!("{}", string), file, line);
}

#[inline(never)]
#[cold]
pub fn begin_panic_fmt(fmt: &fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    let &(file, line) = file_line;

    rust_begin_panic(format_args!("{}", fmt), file, line);
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
/// Required to handle panics
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {
        let _ = exit(129);
    }
}
