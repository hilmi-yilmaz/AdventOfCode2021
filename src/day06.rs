use crate::get_data_from_input_file;

// Parse the input data into a 2D vector of u8's.
fn parse_data(input_data: Vec<String>) -> Vec<Vec<u8>> {

	input_data.iter()
				.map(|number_string| number_string.split(',')
					.map(|number| number.trim().parse::<u8>().unwrap())
					.collect())
				.collect()
}

// Calculate the total number of fish for the amount of days.
fn get_number_of_fish(internal_timer: &mut Vec<usize>, total_days: u32) {

	let mut last_value_at_idx_zero: usize = 0;

	for _ in 1..total_days + 1 {

		// move elements to the left and set idx 8 to 0.
		let mut j: usize = 0;
		while j < 8 {
			internal_timer[j] = internal_timer[j + 1];
			j += 1;
		}
		internal_timer[8] = 0;

		// Move values at idx 0 in the loop before to idx 6 and idx 8.
		if last_value_at_idx_zero != 0 {
			internal_timer[6] += last_value_at_idx_zero;
			internal_timer[8] += last_value_at_idx_zero;
		}

		// Keep track of last value at idx 0.
		last_value_at_idx_zero = internal_timer[0];

		
	}
	
	// Calculate total number of fish and print.
	let num_fish: usize = internal_timer.iter().sum();
	if total_days == 80 {
		println!("Day06 part 1 answer = {}", num_fish);
	} else {
		println!("Day06 part 2 answer = {}", num_fish);
	}
}

pub fn day06(filename: &str) {

	let input_data: Vec<String> = get_data_from_input_file(filename);

	// Contains only numbers from 0 to 8 (including 8) representing internal timer.
	let data: &Vec<u8> = &parse_data(input_data)[0];

	// Create vector with counts of internal timers.
	// The indeces represent the internal timers, the elements represent the counts of fish
	let mut internal_timer: Vec<usize> = vec![0; 9];

	// Set the internal timers of the fish in the internal_timer vector.
	for number in data {
		internal_timer[*number as usize] += 1;
	}

	// Make a copy of internal timer for part 2.
	let mut internal_timer_copy: Vec<usize> = internal_timer.clone();

	// Output the total number of fish for 80 and 256 days (part1 and part2).
	get_number_of_fish(&mut internal_timer, 80);
	get_number_of_fish(&mut internal_timer_copy, 256);
}