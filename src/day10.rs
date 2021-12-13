use crate::get_data_from_input_file;
use std::collections::HashMap;

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
}

fn parse_input_data(input_data: Vec<String>) -> Vec<Vec<char>> {
	input_data.iter()
				.map(|string| string.chars().collect())
				.collect()
}

// Check for one input line if it is correct, corrupt, or incomplete.
fn check_one_input_line(stack: &mut Stack<char>, input_line: &Vec<char>) -> char {
	let mut corrupt_char: char = 'a';
	for chr in input_line.iter() {
		if *chr == '{' || *chr == '[' || *chr == '(' || *chr == '<' {
			stack.push(*chr);
		}
		else if *chr == '}' || *chr == ']' || *chr == ')' || *chr == '>'{
			if *stack.peek().unwrap() == '{' && *chr != '}' {
				corrupt_char = *chr;
				break ;
			}
			else if *stack.peek().unwrap() == '[' && *chr != ']' {
				corrupt_char = *chr;
				break ;
			}
			else if *stack.peek().unwrap() == '(' && *chr != ')' {
				corrupt_char = *chr;
				break ;
			}
			else if *stack.peek().unwrap() == '<' && *chr != '>' {
				corrupt_char = *chr;
				break ;
			}
			else {
				stack.pop();
			}
		}
	}
	corrupt_char
}

pub fn day10(filename: &str) {

	// Get and parse input data to suitable form.
	let input_data: Vec<String> = get_data_from_input_file(filename);
	let parsed_data: Vec<Vec<char>> = parse_input_data(input_data);

	// Create stack data structure
	let mut stack: Stack<char> = Stack::new();

	// Counter for corrupt chars
	let mut counter: Vec<usize> = vec![0; 4];

	// Decode all lines in the input
	for input_line in parsed_data.iter () {
		let corrupt_char: char = check_one_input_line(&mut stack, input_line);
		// if corrupted char is 'a' then the sequence is incomplete or correct.
		// If stack still has elements, then it is incomplete.
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

}