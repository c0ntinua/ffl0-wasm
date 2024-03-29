pub fn cursor_to(r: usize, c : usize) {
	print!("\u{1B}[{};{}H",r,c);
}
pub fn hide_cursor() {
	print!("\u{1B}[?25l");	
}

pub fn clear_screen() {
	print!("\u{1B}[2J");
}

pub fn set_color(r : u8, g : u8, b : u8) {
	print!("\u{1B}[38;2;{};{};{}m",r,g,b);
}
