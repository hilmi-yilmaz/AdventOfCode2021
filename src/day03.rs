use crate::get_data_from_input_file;

// Save the bits from input_data into bits_counts
// e.g. [7, 1, 3, 5, 6], means first column has 7 bits that are 1 etc.
fn get_bits_counts(input_data: &Vec<String>) -> Vec<u32> {

	let mut bits_counts: Vec<u32> = vec![0; input_data[0].len()];
	let mut i: usize = 0;
	for element in input_data {
		for c in element.chars() {
			let bit: u32 = c.to_digit(2).unwrap();
			bits_counts[i] += bit;
			i += 1;
		}
		i = 0;
	}
	bits_counts
}

fn elements_left(active_elements: &Vec<u32>) -> usize {
	let mut count: usize = 0;
	for element in active_elements {
		if *element == 1 {
			count += 1;
		}
	}
	count
}

// determines which bit to keep for column in input_data
fn which_bit_to_keep(input_data: &Vec<String>, active_elements: &Vec<u32>, column: usize, life_support_type: char) -> u8 {

	let mut i: usize = 0;
	let mut bits_that_are_one: usize = 0;
	for element in input_data {
		if active_elements[i] == 1 {
			bits_that_are_one += (element.as_bytes()[column] as usize) - ('0' as usize);
		}
		i += 1;
	}
	if life_support_type == 'o' {
		if bits_that_are_one >= (elements_left(active_elements) / 2) {
			return 1;
		} else {
			return 0;
		}
	} else {
		if bits_that_are_one < (elements_left(active_elements) / 2) {
			return 1;
		} else {
			return 0;
		}
	}
}

fn get_only_active_element(input_data: &Vec<String>, active_elements: &Vec<u32>) -> String {
	
	let mut i: usize = 0;
	for element in active_elements {
		if *element == 1 {
			break;
		}
		i += 1;
	}
	input_data[i].clone()
}

fn convert_binary_string_to_number(binary_string: String) -> u32 {
	
	let mut i: u32 = 0;
	let mut oxygen_rating: u32 = 0; 
	for bit in binary_string.chars().rev() {
		let bit: u32 = (bit as u32) - 48;
		oxygen_rating += bit * ((2 as u32).pow(i));
		i += 1;
	}
	oxygen_rating
}

fn count_active_elements(active_elements: &Vec<u32>) -> u32 {
	
	let mut i: u32 = 0;
	for element in active_elements {
		if *element == 1 {
			i += 1;
		}
	}
	i
}

// gets rating depending on life_support_type, either 'o' or 'c' (oxygen, co2)
fn get_rating(input_data: Vec<String>, life_support_type: char) -> u32
{
	let mut active_elements = vec![1; input_data.len()];
	let bits_in_row: usize = input_data[0].len();
	for column in 0..bits_in_row {
		let bit_to_keep: u8 = which_bit_to_keep(&input_data, &active_elements, column, life_support_type);
		let mut i: usize = 0;
		for element in &input_data {
			if count_active_elements(&active_elements) == 1 {
				break;
			}
			let bit: u8 = element.as_bytes()[column] - ('0' as u8);
			if bit != bit_to_keep {
				active_elements[i] = 0;
			}
			i += 1;
		}
	}
	let only_active_element = get_only_active_element(&input_data, &active_elements);
	let oxygen_rating: u32 = convert_binary_string_to_number(only_active_element);
	oxygen_rating
}

pub fn day03(filename: &str) {
	let input_data: Vec<String> = get_data_from_input_file(filename);
	let len_input: u32 = input_data.len() as u32;
	let mut bits_counts: Vec<u32> = get_bits_counts(&input_data);

	// Set the element to 0 and 1, depending on which one is present more in column.
	for element in &mut bits_counts {
		if *element >= len_input / 2 + 1 {
			*element = 1;
		} else {
			*element = 0;
		}
	}

	// encode the bits in bits_counts
	let mut gamma = 0;
	let mut epsilon = 0;
	let mut i: u32 = 0;
	for element in bits_counts.iter().rev() {
		gamma = gamma + element * ((2 as u32).pow(i));
		epsilon = epsilon + (1 - element) * ((2 as u32).pow(i));
		i += 1;
	}
	println!("Day03 part 1 answer = {}", gamma * epsilon);
	let oxygen_rating: u32 = get_rating(input_data.clone(), 'o');
	let co2_rating: u32 = get_rating(input_data.clone(), 'c');
	println!("Day03 part 2 answer = {}", oxygen_rating * co2_rating);
}
