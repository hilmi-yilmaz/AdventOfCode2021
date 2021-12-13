#![allow(dead_code)]
#![allow(unused_imports)]

// Declare submodules
mod read_lines;
mod day01;
mod day02;
mod day03;
mod	day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

// Use submodules
use read_lines::get_data_from_input_file;
use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use day09::day09;
use day10::day10;

fn main() {
	// day01("inputs/day01/part1_input");
	// day02("inputs/day02/part1_input", "inputs/day02/part2_input");
    // day03("inputs/day03/part1_input");
	// day04("inputs/day04/part1_input");
	// day04("inputs/day04/input_test");
	// day04("inputs/day04/part1_input");
	// day05("inputs/day05/part2_input"); // go into src code of day05, uncomment line 199 to get part1
	// day06("inputs/day06/part1_input");
	// day07("inputs/day07/part1_input");
	// day08("inputs/day08/part1_input");
	// day09("inputs/day09/part1_input");
	day10("inputs/day10/part1_input");
}
