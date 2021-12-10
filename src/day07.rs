use crate::get_data_from_input_file;

fn parse_input_data(input_data: Vec<String>) -> Vec<Vec<usize>> {
	input_data.iter()
				.map(|numbers| numbers.split(',')
					.map(|number| number.trim().parse::<usize>().unwrap())
					.collect())
				.collect()
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

pub fn day07(filename: &str)
{
	// Get input data
	let input_data: Vec<String> = get_data_from_input_file(filename);
	
	// Parse input data
	let data: Vec<usize> = parse_input_data(input_data)[0].clone();
	let mut horizontal_positions: Vec<usize> = data.clone();

	// Get unique horizontal positions by sorting and then using dedup
	horizontal_positions.sort();
	horizontal_positions.dedup();

	// Part 1
	part1(horizontal_positions, data);

}