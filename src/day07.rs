use crate::get_data_from_input_file;

#[derive(Clone, Debug)]
struct Crab {
	position: usize,
	step_fuel_cost: usize,
}

fn parse_input_data(input_data: Vec<String>) -> Vec<Vec<usize>> {
	input_data.iter()
				.map(|numbers| numbers.split(',')
					.map(|number| number.trim().parse::<usize>().unwrap())
					.collect())
				.collect()
}

fn sum_range(number: usize) -> usize {
	let mut sum: usize = 0;
	for i in 1..number + 1 {
		sum += i as usize;
	}
	sum
}

fn part1(horizontal_positions: Vec<usize>, data: Vec<usize>) {

	let mut fuel_cost: Vec<usize> = vec![0; horizontal_positions.len()];
	let mut i: usize = 0;
	for position in &horizontal_positions {

		let mut sum: usize = 0;
		for element in data.iter() {
			sum += (*position as i32 - *element as i32).abs() as usize; // calculation represents fuel from position to element.
		}
		fuel_cost[i] += sum;
		i += 1;
	}

	// Get index of minimal cost
	let min_fuel: usize = *fuel_cost.iter().min().unwrap();
	println!("Day07 part 1 answer = {}", min_fuel);
}

fn part2(horizontal_positions: Vec<usize>, data: Vec<usize>) {

	let mut fuel_cost: Vec<usize> = vec![0; horizontal_positions.len()];
	let mut i: usize = 0;
	for position in &horizontal_positions {
		let mut sum: usize = 0;
		for crab in data.iter() {
			sum += sum_range((*position as i32 - *crab as i32).abs() as usize); // calculation represents fuel from position to element.
		}
		fuel_cost[i] += sum;
		i += 1;
	}

	// Get index of minimal cost
	let min_fuel: usize = *fuel_cost.iter().min().unwrap();
	println!("Day07 part 2 answer = {}", min_fuel);
}

pub fn day07(filename: &str)
{
	// Get input data + a copy for part2
	let input_data: Vec<String> = get_data_from_input_file(filename);
	
	// Parse input data
	let data: Vec<usize> = parse_input_data(input_data)[0].clone();
	let data_clone: Vec<usize> = data.clone();

	// Get unique horizontal positions by finding min and max value of all positions
	let min_position: usize = *data.iter().min().unwrap();
	let max_position: usize = *data.iter().max().unwrap();
	let horizontal_positions: Vec<usize> = (0..(max_position - min_position + 1)).collect();

	// Copy data for part 2
	let horizontal_positions_clone: Vec<usize> = horizontal_positions.clone();
	
	// Part 1
	part1(horizontal_positions, data);

	// Part 2
	part2(horizontal_positions_clone, data_clone);
}