use crate::get_data_from_input_file;
use std::collections::HashMap;

#[derive(Debug)]
struct Stack<T> {
	stack: Vec<T>
}

impl<T> Stack<T> {
	fn new() -> Self {
		Stack { stack: Vec::new() }
	}

	fn pop(&mut self) -> Option<T> {
		self.stack.pop()
	}

	fn push(&mut self, item: T) {
		self.stack.push(item)
	}

	fn is_empty(&self) -> bool {
		self.stack.is_empty()
	}

	fn lenght(&self) -> usize {
		self.stack.len()
	}

	fn peek(&self) -> Option<&T> {
		self.stack.last()
	}

	fn clear(&mut self) {
		self.stack.clear();
	}
}

fn parse_input_data(input_data: Vec<String>) -> Vec<Vec<char>> {
	input_data.iter()
				.map(|string| string.chars().collect())
				.collect()
}

// Check for one input line if it is correct, corrupt, or incomplete.
fn check_one_input_line(stack: &mut Stack<char>, input_line: &Vec<char>) -> char {

	let mut corrupt_char: char = 'a';
	let mut flag = 0;
	for chr in input_line.iter() {
		if *chr == '{' || *chr == '[' || *chr == '(' || *chr == '<' {
			stack.push(*chr);
		}
		else if *chr == '}' || *chr == ']' || *chr == ')' || *chr == '>'{
			if *stack.peek().unwrap() == '{' && *chr != '}' {
				corrupt_char = *chr;
				flag = 1;
				break ;
			}
			else if *stack.peek().unwrap() == '[' && *chr != ']' {
				corrupt_char = *chr;
				flag = 1;
				break ;
			}
			else if *stack.peek().unwrap() == '(' && *chr != ')' {
				corrupt_char = *chr;
				flag = 1;
				break ;
			}
			else if *stack.peek().unwrap() == '<' && *chr != '>' {
				corrupt_char = *chr;
				flag = 1;
				break ;
			}
			else {
				stack.pop();
			}
		}
	}
	if flag == 1 {
		stack.clear();
	}
	corrupt_char
}

fn incomplete_line_score(chars_vec: &Vec<char>) -> usize {

	let mut score: usize = 0;
	
	for chr in chars_vec.iter().rev() {
		score *= 5;
		if *chr == '[' {
			score += 2;
		} else if *chr == '(' {
			score += 1;
		} else if *chr == '{' {
			score += 3;
		} else if *chr == '<' {
			score += 4;
		}
	}

	score
}

// fn complete_line(chars_vec: &Vec<char>) {
	
// 	for chr in chars_vec.iter().rev() {
// 		if (*chr == '[') {

// 		}
// 	}
// }

pub fn day10(filename: &str) {

	// Get and parse input data to suitable form.
	let input_data: Vec<String> = get_data_from_input_file(filename);
	let parsed_data: Vec<Vec<char>> = parse_input_data(input_data);

	// Counter for corrupt chars
	let mut counter: Vec<usize> = vec![0; 4];

	// Save incomplete lines
	let mut incomplete: Vec<Vec<char>> = Vec::new();

	// Decode all lines in the input
	for input_line in parsed_data.iter () {

		// Create stack data structure
		let mut stack: Stack<char> = Stack::new();
		
		// if corrupted char is 'a' then the sequence is incomplete or correct.
		// If stack still has elements, then it is incomplete.
		let corrupt_char: char = check_one_input_line(&mut stack, input_line);

		// Save all incomplete stacks for part2, to finish them myself
		if !stack.is_empty() {
			incomplete.push(stack.stack);
		}

		if corrupt_char == '}' {
			counter[0] += 1;
		}
		else if corrupt_char == ']' {
			counter[1] += 1;
		}
		else if corrupt_char == ')' {
			counter[2] += 1;
		}
		else if corrupt_char == '>' {
			counter[3] += 1;
		}
	}

	// Create mapping from corrupted char to score
	let scores: Vec<usize> = vec![1197, 57, 3, 25137];

	// Calculate syntax error score
	let syntax_error_score: usize = counter[0] * scores[0] + counter[1] * scores[1] + 
										counter[2] * scores[2] + counter[3] * scores[3];

	println!("Day10 part1 answer = {}", syntax_error_score);

	let mut total_score: Vec<usize> = Vec::new();
	for line in incomplete.iter() {
		total_score.push(incomplete_line_score(line));
	}
	total_score.sort();

	let idx_middle_value = total_score.len() / 2;
	let final_score: usize = total_score[idx_middle_value];
	println!("Day10 part 2 answer = {}", final_score);
}