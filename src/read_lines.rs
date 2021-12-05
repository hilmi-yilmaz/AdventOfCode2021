use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// create an iterator over the lines of filename.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

// Return the input data in a Vector, each line in the file being an element.
pub fn get_data_from_input_file(filename: &str) -> Vec<String> {
	let mut input_data: Vec<String> = Vec::new();
	if let Ok(lines) = read_lines(filename) {
		for line in lines {
			if let Ok(input_line) = line {
				input_data.push(input_line);
			}
		}
	}
	input_data
}