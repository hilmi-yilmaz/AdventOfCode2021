use crate::get_data_from_input_file;

use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
	x: u32,
	y: u32,
}

#[derive(Debug)]
struct Line {
	start: Coordinate,
	end: Coordinate,
}

// fn format_input_data(input_data: Vec<String>) -> Vec<Vec<String>> {
// 	input_data.iter().
// 		map(|line_segment| line_segment.split_whitespace()
// 			.map(|coordinates| coordinates.trim().to_string())
// 			.filter(|element| element.chars().nth(0).unwrap().is_digit(10))
// 			.collect())
// 		.collect()
// }

// //format the data into 3D vector
// fn format_input_data(input_data: Vec<String>) -> Vec<Vec<Vec<u32>>> {
// 	input_data.iter().
// 		map(|line_segment| line_segment.split_whitespace()
// 			.map(|coordinates| coordinates.trim().to_string())
// 			.filter(|element| element.chars().nth(0).unwrap().is_digit(10))
// 			.map(|coordinates| coordinates.split(',')
// 				.map(|nb| nb.trim().parse::<u32>().unwrap())
// 				.collect::<Vec<u32>>())
// 			.collect::<Vec<Vec<u32>>>())
// 		.collect::<Vec<Vec<Vec<u32>>>>()
// }

//format the data into 1D vector containing Line.
fn format_input_data(input_data: Vec<String>) -> Vec<Line> {
	input_data.iter().
		map(|line_segment| line_segment.split_whitespace()
			.map(|coordinates| coordinates.trim().to_string())
			.filter(|element| element.chars().nth(0).unwrap().is_digit(10))
			.map(|coordinates| coordinates.split(',')
				.map(|nb| nb.trim().parse::<u32>().unwrap())
				.collect::<Vec<u32>>())
			.collect::<Vec<Vec<u32>>>().iter()
										.map(|coordinate| Coordinate {x: coordinate[0], y: coordinate[1]})
										.collect::<Vec<Coordinate>>())
		.collect::<Vec<Vec<Coordinate>>>().iter()
											.map(|line| Line {start: line[0].clone(), end: line[1].clone()})
											.collect::<Vec<Line>>()
}

fn get_min_and_max_values(lines_vec: &Vec<Line>) -> (usize, usize) {

	// let x_min: usize = lines_vec.iter()
	// 							.map(|line| {if line.start.x > line.end.x {line.end.x} else {line.start.x}})
	// 							.min()
	// 							.unwrap().try_into().unwrap(); // try_into convert u32 into usize and panics if it doesn't fit.

	// let y_min: usize = lines_vec.iter()
	// 							.map(|line| {if line.start.y > line.end.y {line.end.y} else {line.start.y}})
	// 							.min()
	// 							.unwrap().try_into().unwrap();

	let x_max: usize = lines_vec.iter()
								.map(|line| {if line.start.x < line.end.x {line.end.x} else {line.start.x}})
								.max()
								.unwrap().try_into().unwrap(); // try_into convert u32 into usize and panics if it doesn't fit.

	let y_max: usize = lines_vec.iter()
								.map(|line| {if line.start.y < line.end.y {line.end.y} else {line.start.y}})
								.max()
								.unwrap().try_into().unwrap();

	(x_max, y_max)
}

fn filter_horizontal_and_vertical_lines(lines_vec: Vec<Line>) -> Vec<Line> {
	let lines_vec: Vec<Line> = lines_vec.into_iter()
			.filter(|line| line.start.x == line.end.x || line.start.y == line.end.y) // vertical lines
			.collect();
	lines_vec
}

fn draw_horizontal_line(diagram: &mut Vec<Vec<u8>>, line: &Line) {

	let mut x_start: usize = line.start.x as usize;
	let	moves: usize = (line.start.x as isize - line.end.x as isize).abs() as usize;
	if line.start.x > line.end.x {
		x_start = line.end.x as usize;
	}
	for j in 0..(&diagram[line.start.y as usize]).len() {
		if 	j >= x_start && j <= x_start + moves {
			diagram[line.start.y as usize][j] += 1;
		}
	}
}

fn draw_vertical_line(diagram: &mut Vec<Vec<u8>>, line: &Line) {

	//println!("line.start.x = {}", line.start.x);
	let mut y_start: usize = line.start.y as usize;
	let moves: usize = (line.start.y as isize - line.end.y as isize).abs() as usize;
	if line.start.y > line.end.y {
		y_start = line.end.y as usize;
	}
	for j in 0..diagram.len() {
		if j >= y_start && j <= y_start + moves {
			diagram[j][line.start.x as usize] += 1;
		}
	}
}



pub fn day05(filename: &str) {
	
	// Get input data
	let input_data: Vec<String> = get_data_from_input_file(filename);

	// Format the data
	let lines_vec: Vec<Line> = format_input_data(input_data);

	// Get the minimum and maximum x and y coordinates to create a diagram (Vec<Vec<u8>>)
	let (x_max, y_max): (usize, usize) = get_min_and_max_values(&lines_vec);

	// Create the diagram, index using diagram[y][x]
	let mut diagram: Vec<Vec<u8>> = vec![vec![0; x_max + 1]; y_max + 1];

	// Filter horizontal and vertical lines
	let lines_vec: Vec<Line> = filter_horizontal_and_vertical_lines(lines_vec);

	// Draw lines on diagram
	for line in &lines_vec {
		if line.start.y == line.end.y {
			draw_horizontal_line(&mut diagram, line);
		} 
		else if line.start.x == line.end.x {
			draw_vertical_line(&mut diagram, line);
		}
	}

	// Extract element=2 in diagram
	// let sum: Vec<u8> = diagram.iter().flat_map(|x| x.to_vec()).collect::<Vec<u8>>();
	// let count: usize = sum.iter().cloned().filter(|&nb| nb == 2).collect::<Vec<u8>>().len();

	let count: u32 = diagram.into_iter()
								.map(|row| row.iter()
									.filter(|&&nb| nb >= 2)
									.count() as u32)
								.collect::<Vec<u32>>()
								.iter()
								.sum();

	println!("Day05 part1 answer = {:?}", count);

	// Check diagram
	// for row in &diagram {
	// 	println!("{:?}", row);
	// }
}