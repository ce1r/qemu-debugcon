#![no_std]

use core::arch::asm;
use core::fmt;
use core::fmt::Write;

#[derive(Debug)]
pub struct DebugCon;

impl DebugCon {
    const DEBUGCON_PORT: u16 = 0xe9;
}

impl fmt::Write for DebugCon {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                asm!(
                    "out dx, al",
                    in("dx") Self::DEBUGCON_PORT,
                    in("al") byte,
                    options(nomem, nostack, preserves_flags),
                );
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! debugcon {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debugconln {
    () => ($crate::debugcon!("\n"));
    ($($arg:tt)*) => ($crate::debugcon!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    let _ = DebugCon.write_fmt(args);
}

#[cfg(feature = "log")]
impl log::Log for DebugCon {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut con = DebugCon;
        let _ = writeln!(
            con,
            "[ {}]: {}@{}: {}",
            record.level(),
            record.file().unwrap_or("<unknown file>"),
            record.line().unwrap_or(0),
            record.args()
        );
    }

    fn flush(&self) {}
}

#[cfg(feature = "log")]
pub fn init_logger() {
    log::set_logger(&DebugCon).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
