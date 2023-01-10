pub fn cursor_to(r: usize, c : usize) {
	print!("\u{1B}[{};{}H",r,c);
}
pub fn hide_cursor() {
	print!("\u{1B}[?25l");	
}
pub fn switch_to_alternate_buffer() {
	print!("\u{1B}[?1049h");
}
pub fn switch_to_primary_buffer() {
	print!("\u{1B}[?1049l");
}

pub fn clear_screen() {
	print!("\u{1B}[2J");
}
pub fn start_bold_text() {
	print!("\u{1B}[1m");
}
pub fn start_reverse_text() {
	print!("\u{1B}[7m");
}
pub fn stop_bold_text() {
	print!("\u{1B}[22m");
}
pub fn stop_reverse_text() {
	print!("\u{1B}[27m");
}

pub fn set_color(r : u8, g : u8, b : u8) {
	print!("\u{1B}[38;2;{};{};{}m",r,g,b);
}

pub fn rgb_string(r : u8, g : u8, b : u8) -> String {
	format!("\x1b[38;2;{};{};{}m",r,g,b)
}