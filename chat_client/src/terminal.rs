pub struct Terminal;

impl Terminal {
    pub fn move_cursor_down(lines: usize) {
        print!("\x1B[{}B", lines);
    }

    pub fn move_cursor_up(lines: usize) {
        if lines == 1 {
            print!("\x1BM"); // move cursor 1 line up
        } else {
            print!("\x1B{}A", lines); // move cursor 1 line up
        }
    }

    pub fn move_cursor_right(columns: usize) {
        print!("\x1B[{}C", columns);
    }

    pub fn move_cursor_left(columns: usize) {
        print!("\x1B[{}D", columns);
    }

    pub fn move_cursor_to_column(column: usize) {
        print!("\x1B[{}G", column);
    }

    pub fn move_cursor_beggining_up(lines: usize) {
        print!("\x1B[{}F", lines);
    }

    pub fn erase_screen() {
        print!("\x1B[2J");
    }

    pub fn erase_to_end_of_line() {
        print!("\x1B[0K");
    }
}