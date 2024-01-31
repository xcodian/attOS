use core::str;

use crate::writer::Writer;

const VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;

pub struct VgaTextModeWriter {
    pub row: usize,
    pub column: usize,
}

impl VgaTextModeWriter {
    pub fn new() -> Self {
        Self { row: 0, column: 0 }
    }

    pub fn ptr_at(row: usize, column: usize) -> *mut u8 {
        unsafe { VIDEO_MEM.add(row * 160 + column * 2) }
    }

    pub fn clear(&mut self) {
        let empty_row = unsafe { str::from_utf8_unchecked(&[0u8; 80]) };

        for row in 0..25 {
            self.print_at(empty_row, row, 0)
        }

        self.row = 0;
        self.column = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let empty_row = str::from_utf8(&[0u8; 80]).unwrap();
        self.print_at(empty_row, row, 0)
    }
}

impl Default for VgaTextModeWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer for VgaTextModeWriter {
    fn print(&mut self, s: &str) {
        self.print_at(s, self.row, self.column);
        self.column += s.len();
        self.column %= 80;
    }

    fn println(&mut self, s: &str) {
        self.print_at(s, self.row, self.column);
        self.column = 0;
        self.row += 1;
        if self.row >= 25 {
            self.row = 0;
        }
    }

    fn print_at(&mut self, s: &str, row: usize, column: usize) {
        let ptr = Self::ptr_at(row, column);

        for (i, c) in s.bytes().enumerate() {
            unsafe {
                *ptr.add(i * 2) = c;
            }
        }
    }
}
