use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
// use std::env;

#[derive(Debug)]
struct Position {
	horizontal: i32,
	vertical: i32,
	aim: i32,
}

impl Position {
	fn multiply_position(self) -> i32 {
		self.horizontal * self.vertical
	}
}

// Implement Defailt trait for to initialize all data in Position struct to 0.
impl Default for Position {
	fn default() -> Position {
		Position{horizontal: 0, vertical: 0, aim: 0}
	}
}

// create an iterator over the lines of filename.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

//Part 1
fn day02_part1(filename: &str) {
	let mut data = Position::default();
	if let Ok(lines) = read_lines(filename) {
		for line in lines {
			if let Ok(instruction) = line {
				let split: Vec<&str> = instruction.split(' ').collect();
				let b: char = split[0].as_bytes()[0] as char;
				let number: i32 = split[1].parse().expect("Should be a number");
				if b == 'f' {
					data.horizontal += number;
				} else if b == 'u' {
					data.vertical -= number;
				} else if b == 'd' {
					data.vertical += number;
				}
			}
		}
	}
	let answer = data.multiply_position();
	println!("Day02 part 1 answer = {}", answer);

}

// Part 2
fn day02_part2(filename: &str) {
	let mut data = Position::default();
	if let Ok(lines) = read_lines(filename) {
		for line in lines {
			if let Ok(instruction) = line {
				let split: Vec<&str> = instruction.split(' ').collect();
				let b: char = split[0].as_bytes()[0] as char;
				let number: i32 = split[1].parse().expect("Should be a number");
				if b == 'f' {
					data.horizontal += number;
					data.vertical += number * data.aim;
				} else if b == 'u' {
					data.aim -= number;
				} else if b == 'd' {
					data.aim += number;
				}
			}
		}
	}
	let answer = data.multiply_position();
	println!("Day02 part 2 answer = {}", answer);
}

pub fn day02(filename1: &str, filename2: &str) {
	day02_part1(filename1);
	day02_part2(filename2);
}