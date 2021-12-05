use crate::get_data_from_input_file;

fn get_to_be_drawn_number(input_data: &Vec<String>) -> Vec<i8> {

	input_data[0].split(',')
				.map(|s| s.parse::<i8>()
				.unwrap())
				.collect()
}

fn strip_input_from_newlines(input_data: &mut Vec<String>) {
	input_data.retain(|s| !s.is_empty())
}

fn create_all_boards(chunks_input_data: &Vec<Vec<String>>) -> Vec<Vec<Vec<i8>>> {
	chunks_input_data.iter()
		.map(|board| board.iter()
			.map(|row| row.trim().split_whitespace()
				.map(|number| number.trim().parse::<i8>().unwrap())
				.collect::<Vec<i8>>())
			.collect())
		.collect()
}

fn mark_element_on_single_board(single_board: &Vec<Vec<i8>>, drawn_number: i8) -> Vec<Vec<i8>> {
	single_board.iter()
		.map(|row| row.iter()
			.map(|&number| {if number == drawn_number {-1} else {number}})
			.collect())
		.collect()
}

fn check_board_for_completion(single_board: &Vec<Vec<i8>>) -> bool {
	
	// check rows
	for row in single_board.iter() {
		if *row == vec![-1; 5] {
			return true;
		}
	}

	// check columns
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut count: usize = 0;
	let len = single_board.len();
	while i < len {
		while j < len {
			if single_board[j][i] == -1 {
				count += 1;
			}
			j += 1;
		}
		if count == 5 {
			return true;
		}
		count = 0;
		j = 0;
		i += 1;
	}
	false
}

fn get_sum_unmarked_numbers(single_board: &Vec<Vec<i8>>) -> u32 {

	let mut sum: u32 = 0;
	for row in single_board {
		for element in row {
			if *element != -1 {
				sum += *element as u32;
			}
		}
	}
	sum
}

fn part01(mut all_boards: Vec<Vec<Vec<i8>>>, to_be_drawn_numbers: &Vec<i8>) {
	let mut answer: u32 = 0;
	for drawn_number in to_be_drawn_numbers {
		all_boards = all_boards.iter()
								.map(|board| mark_element_on_single_board(&board, *drawn_number))
								.collect();
		for board in &all_boards {
			if check_board_for_completion(&board) {
				answer = get_sum_unmarked_numbers(board) * (*drawn_number as u32);
				break ;
			}
		}
		if answer != 0 {
			break;
		}
		
	}
	println!("Day04 part 1 answer = {}", answer);
}

fn part02(mut all_boards: Vec<Vec<Vec<i8>>>, to_be_drawn_numbers: &Vec<i8>) {
	let mut answer: u32 = 0;
	let mut who_won: Vec<i8> = vec![0; all_boards.len()];
	for drawn_number in to_be_drawn_numbers {
		all_boards = all_boards.iter()
								.map(|board| mark_element_on_single_board(&board, *drawn_number))
								.collect();
		let mut i: usize = 0;
		for board in &all_boards {
			if check_board_for_completion(&board) {
				who_won[i] = 1;
				if who_won.iter().sum::<i8>() == (all_boards.len() as i8) {
					answer += get_sum_unmarked_numbers(board) * (*drawn_number as u32);
					break ;
				}
			}
			i += 1;
		}
		if answer != 0 {
			break ;
		}
	}
	println!("Day04 part 2 answer = {}", answer);
}


pub fn day04(filename: &str) {
	let mut input_data: Vec<String> = get_data_from_input_file(filename);

	// Strip input from newlines
	strip_input_from_newlines(&mut input_data);

	// Get to be drawn numbers
	let to_be_drawn_numbers: Vec<i8> = get_to_be_drawn_number(&input_data);
	input_data.remove(0);

	// Create 2D vector containing the lines of the boards
	let chunks_input_data: Vec<Vec<String>> = input_data.chunks(5)
														.map(|board| board.to_vec())
														.collect();

	// Get boards
	let all_boards: Vec<Vec<Vec<i8>>> = create_all_boards(&chunks_input_data);

	// Part 1
	let copy_all_boards: Vec<Vec<Vec<i8>>> = all_boards.clone();
	part01(copy_all_boards, &to_be_drawn_numbers);

	// Part 2
	let copy_all_boards: Vec<Vec<Vec<i8>>> = all_boards.clone();
	part02(copy_all_boards, &to_be_drawn_numbers);
}