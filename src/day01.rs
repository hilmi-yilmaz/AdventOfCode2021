use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// create an iterator over the lines of filename.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?; //open a file, filename needs to be of type P, which is an alias for the type AsRef<Path>.
	Ok(io::BufReader::new(file).lines()) // bufreader is a struct which adds buffering to any reader.
}

// Returns vector containing all numbers from the input file
fn get_data_with_file_input(input_file: String) -> Vec<i32> {

	// create a vector to put the integers in.
	let mut data: Vec<i32> = Vec::new();

	// loop over lines in input file and save in data vector.
	if let Ok(lines) = read_lines(input_file) {
		for line in lines {
			if let Ok(number) = line {
				data.push(number.trim().parse().unwrap());
			}
		}
	}
	data
}

// Returns the amount of increases in the data vector.
fn get_increases(data: &Vec<i32>) -> i32
{
	let mut	increases = 0;
	let len = data.len();
	for idx in 0..len - 1 {
		if data[idx] < data[idx + 1] {
			increases += 1;
		}
	}
	increases
}

pub fn day01(filename: &str) {

	// let input_file = get_input_file_name();
	let data: Vec<i32> = get_data_with_file_input(filename.to_string());

	// Part 1
	let increases = get_increases(&data);
	println!("Day01 part 1 answer = {}", increases);

	// Part 2
	let mut	window_data: Vec<i32> = Vec::new();
	let window_width = 3;
	let mut	sum_of_window: i32 = 0;
	for idx in 0..(data.len() - window_width + 1) {
		if idx == 0 {
			for k in 0..window_width {
				sum_of_window += data[k];
			}
		} else {
			sum_of_window = sum_of_window - data[idx - 1] + data[idx + window_width - 1];
		}
		window_data.push(sum_of_window);
	}

	let increases = get_increases(&window_data);
	println!("Day01 part 2 answer = {}", increases);


}

// Some extra notes:
// new(file) creates a BufReader<R> with a default buffer capacity of 8KB.
// lines() returns an iterator over the lines of this reader.
// The iterator returned from this function will yield instances of io::Result<String>.
// Each string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.