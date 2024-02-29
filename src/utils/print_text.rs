use std::io::{self, Write};
use console::style;

#[warn(dead_code)]
pub enum Colors {
	Red,
	Green,
	Yellow,
}

pub fn print_text(text: &str, color: Colors) {
	let result = match color {
    Colors::Red => style(text.to_string()).red().bold(),
    Colors::Green => style(text.to_string()).green().bold(),
		Colors::Yellow => style(text.to_string()).yellow().bold(),
	};
	print!("{} ", result);
	io::stdout().flush().unwrap();
}