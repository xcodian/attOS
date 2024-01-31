use core::{
    fmt::{self, Write},
    str,
};

use crate::{writer::Writer, WRITER};

const VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;

pub struct VgaTextModeWriter {
    pub row: usize,
    pub column: usize,
}

impl VgaTextModeWriter {
    pub const fn new() -> Self {
        Self { row: 0, column: 0 }
    }

    pub fn ptr_at(row: usize, column: usize) -> *mut u8 {
        unsafe { VIDEO_MEM.add(row * 160 + column * 2) }
    }

    pub fn clear(&mut self) {
        let empty_row = unsafe { str::from_utf8_unchecked(&[0u8; 80]) };

        for row in 0..25 {
            self.row = row;
            self.column = 0;
            self.print(empty_row)
        }

        self.row = 0;
        self.column = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let empty_row = str::from_utf8(&[0u8; 80]).unwrap();
        self.row = row;
        self.column = 0;
        self.print(empty_row)
    }

    fn wrap(&mut self) {
        self.row += 1;
        if self.row >= 25 {
            self.row = 0;
        }

        self.column = 0;
    }
}

impl Default for VgaTextModeWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer for VgaTextModeWriter {
    fn print(&mut self, s: &str) {
        for c in s.bytes() {
            if c == '\n' as u8 {
                self.wrap();
            } else {
                unsafe {
                    *Self::ptr_at(self.row, self.column) = c;
                }
                self.column += 1;
                if self.column >= 80 {
                    self.wrap();
                }
            }
        }
    }
}

impl fmt::Write for VgaTextModeWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::textmode::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        let _ = WRITER.write_fmt(args);
    }
}
