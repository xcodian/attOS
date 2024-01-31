pub trait Writer {
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str);
    fn print_at(&mut self, s: &str, row: usize, column: usize);
}