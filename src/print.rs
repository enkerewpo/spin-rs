use core::fmt::{self, Write};

/// The base address of the APB_CFG_HEADER
const _APB_CFG_HEADER: u64 = 0xfe_0000_1000;
/// The base address of the UART0
pub const APB_UART0_BASE: u64 = 0x1FE001E0;

/// print a string to uart with raw c function
pub fn __print_raw(s: &str) {
  extern "C" {
    fn print_char(c: u8);
  }
  for c in s.bytes() {
    unsafe {
      print_char(c);
    }
  }
}

struct Writer;

impl Write for Writer {
  // _print call this
  fn write_str(&mut self, s: &str) -> fmt::Result {
    __print_raw(s);
    extern "C" {
      fn tb_print_char(c: u8);
    }
    for c in s.bytes() {
      unsafe {
        tb_print_char(c);
      }
    }
    Ok(())
  }
}

/// core print function
pub fn _print(args: fmt::Arguments) {
  Writer.write_fmt(args).unwrap();
}

/// print a string to uart with color
#[macro_export]
macro_rules! with_color {
  ($color_code:expr, $($arg:tt)*) => {{
      format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
  }};
}

/// print a string to uart without color
#[macro_export]
macro_rules! without_color {
  ($($arg:tt)*) => {{
      format_args!("{}", format_args!($($arg)*))
  }};
}

/// print
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// println
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n\r"));
    ($($arg:tt)*) => ($crate::print!("{}\n\r", format_args!($($arg)*)));
}

/// info
#[macro_export]
macro_rules! _info {
  ($($arg:tt)*) => ({
    $crate::print::_print($crate::without_color!("[INFO |{}:{}]: {}\n\r", file!(), line!(), format_args!($($arg)*)));
  });
}

/// error
#[macro_export]
macro_rules! _error {
  ($($arg:tt)*) => ({
    $crate::print::_print($crate::without_color!("[ERROR|{}:{}]: {}\n\r", file!(), line!(), format_args!($($arg)*)));
  });
}
