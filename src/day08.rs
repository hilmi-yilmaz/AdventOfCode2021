use crate::get_data_from_input_file;

fn parse_data(input_data: Vec<String>) -> Vec<Vec<String>> {

	input_data.iter()
				.map(|encoding| encoding.split('|')
					.map(|split| split.trim().to_string())
					.collect::<Vec<String>>())
				.collect()
}

fn get_all_after_pipe_sign(data: &mut Vec<Vec<String>>) -> Vec<String> {

	data.iter_mut()
			.map(|encoding| encoding.remove(1))
			.collect()
}

// Get encoding string, also converts to chars vector
fn get_all_before_pipe_sign(encoded_string: String) -> Vec<Vec<char>> {

	encoded_string.split_whitespace()
					.map(|string| string.chars().collect())
					.collect()
}

fn part1(parsed_data: &mut Vec<Vec<String>>) {

	let parsed_data: Vec<String> = get_all_after_pipe_sign(parsed_data);

	let one_d_parsed_data: Vec<String> = parsed_data.iter()
														.map(|sentence| sentence.split_whitespace()
															.map(|word| word.to_string())
															.collect())
														.collect::<Vec<Vec<String>>>()
														.into_iter()
														.flatten()
														.collect();

	// Count amount of 1, 4, 7, 8 digits by counting the unique number of segments they contain.
	let count: usize = one_d_parsed_data.iter().filter(|&word| word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7).count();

	println!("Day08 part 1 answer = {}", count);
}

fn ascii_to_usize(character: char) -> usize {
	character as usize - 'a' as usize
}

fn get_digit_with_size_is_number(encoded_data: &Vec<Vec<char>>, number: usize) -> Vec<Vec<char>> {

	encoded_data.iter()
				.filter(|&char_vecter| char_vecter.len() == number)
				.cloned()
				.collect()
}

fn get_new_mapping_values(mapping: Vec<char>, keep_in_mapping: Vec<char>) -> Vec<char> {

	mapping.iter()
			.filter(|&&chr| keep_in_mapping.iter().any(|&char_to_stay| char_to_stay == chr))
			.cloned()
			.collect()
}

fn decode_one_line(encoded_data: &Vec<Vec<char>>, mapping: &mut Vec<Vec<char>>) -> Vec<char> {
	// Get digit with 2 segments (this is number 1) --------------------------------------------------------------------------------------------------------------------
	let two_segments: Vec<Vec<char>> = get_digit_with_size_is_number(&encoded_data, 2);
	// println!("\ntwo_segments:\n{:?}", two_segments);

	// Update the mapping
	mapping[ascii_to_usize('c')] = get_new_mapping_values(mapping[ascii_to_usize('c')].clone(), two_segments[0].clone());
	mapping[ascii_to_usize('f')] = get_new_mapping_values(mapping[ascii_to_usize('f')].clone(), two_segments[0].clone());
	// println!("\nMapping after first update:\n{:?}", mapping);

	// Get digit with 3 segments (this is number 7) --------------------------------------------------------------------------------------------------------------------
	let three_segments: Vec<Vec<char>> = get_digit_with_size_is_number(&encoded_data, 3);
	// println!("\nthree_segments:\n{:?}", three_segments);

	// Update the mapping
	mapping[ascii_to_usize('c')] = get_new_mapping_values(mapping[ascii_to_usize('c')].clone(), three_segments[0].clone());
	mapping[ascii_to_usize('f')] = get_new_mapping_values(mapping[ascii_to_usize('f')].clone(), three_segments[0].clone());
	mapping[ascii_to_usize('a')] = get_new_mapping_values(mapping[ascii_to_usize('a')].clone(), three_segments[0].clone());
	
	// Make conclusions using the current mapping
	mapping[ascii_to_usize('a')] = mapping[ascii_to_usize('a')].iter()
																.filter(|&&chr| !mapping[ascii_to_usize('c')].iter().any(|&char_to_stay| char_to_stay == chr))
																.cloned()
																.collect();
	// println!("\nMapping after second update:\n{:?}", mapping);

	// Get digit with 4 segments (this is number 4) --------------------------------------------------------------------------------------------------------------------
	let four_segments: Vec<Vec<char>> = get_digit_with_size_is_number(&encoded_data, 4);
	// println!("\nfour_segments:\n{:?}", four_segments);

	// Update the mapping
	mapping[ascii_to_usize('c')] = get_new_mapping_values(mapping[ascii_to_usize('c')].clone(), four_segments[0].clone());
	mapping[ascii_to_usize('f')] = get_new_mapping_values(mapping[ascii_to_usize('f')].clone(), four_segments[0].clone());
	mapping[ascii_to_usize('b')] = get_new_mapping_values(mapping[ascii_to_usize('b')].clone(), four_segments[0].clone());
	mapping[ascii_to_usize('d')] = get_new_mapping_values(mapping[ascii_to_usize('d')].clone(), four_segments[0].clone());

	// Make conclusions using the current mapping
	mapping[ascii_to_usize('b')] = mapping[ascii_to_usize('b')].iter()
																.filter(|&&chr| !mapping[ascii_to_usize('c')].iter().any(|&char_to_stay| char_to_stay == chr))
																.cloned()
																.collect();

	mapping[ascii_to_usize('d')] = mapping[ascii_to_usize('d')].iter()
																.filter(|&&chr| !mapping[ascii_to_usize('c')].iter().any(|&char_to_stay| char_to_stay == chr))
																.cloned()
																.collect();

	// println!("\nMapping after third update:\n{:?}", mapping);

	// Get digit with 5 segments (this are number 2, 3, 5) -------------------------------------------------------------------------------------------------------------
	let five_segments: Vec<Vec<char>> = get_digit_with_size_is_number(&encoded_data, 5);
	// println!("\nfive_segments:\n{:?}", five_segments);

	// Find out which of the 2 letters are different
	let result: (Vec<Vec<char>>, Vec<char>) = get_differing_elements_in_vectors(five_segments.clone());

	let difference_two_and_five = result.0;
	let number_three = result.1;

	// println!("\ndifference_two_and_five:\n{:?}", difference_two_and_five);
	// println!("\nnumber_three:\n{:?}", number_three);

	// Update the mapping
	mapping[ascii_to_usize('b')] = vec![difference_two_and_five[0][0], difference_two_and_five[1][0]];
	mapping[ascii_to_usize('e')] = vec![difference_two_and_five[0][0], difference_two_and_five[1][0]];

	// println!("\nMapping after fourth update:\n{:?}", mapping);

	// Find element that is in 4, but not in 3, then I have mapping for real 'b'
	let mut char_in_4_not_in_3: char = number_three[0];
	for chr in four_segments[0].iter() {
		if !number_three.iter().any(|&find| find == *chr) {
			char_in_4_not_in_3 = *chr;
			break;
		}
	}
	// println!("\nchar_in_4_not_in_3 = {}", char_in_4_not_in_3);

	// Update mapping (set b to char_in_4_not_in_3, remove all char_in_4_not_in_3 for the rest)
	
	let mut mapping: Vec<Vec<char>> = mapping.iter()
			.map(|map| map.iter()
				.filter(|&&chr| chr != char_in_4_not_in_3)
				.cloned()
				.collect())
			.collect();

	mapping[ascii_to_usize('b')] = vec![char_in_4_not_in_3];
	// println!("\nMapping after fifth update:\n{:?}", mapping);

	// Char in 2 not in one. This will give mapping for f
	// find number 2 first, has mapping['e'] in it.
	let five_segments_flatten: Vec<char> = five_segments.iter().flat_map(|string| string.iter()).cloned().collect();
	let number_2_unique_char_idx: usize = five_segments_flatten.iter().position(|&chr| chr == mapping[ascii_to_usize('e')][0]).unwrap() / 5;
	let number_two: Vec<char> = five_segments[number_2_unique_char_idx].clone();
	// println!("\nnumber_two = {:?}", number_two);

	// find char that is in 1, not in 2.
	let mut mapping_for_f_is: char = 'a';
	let mut mapping_for_c_is: char = 'a';
	for char_in_one in two_segments[0].iter() {
		if !number_two.iter().any(|&char_in_two| char_in_two == *char_in_one) {
			mapping_for_f_is = *char_in_one;
		} else {
			mapping_for_c_is = *char_in_one;
		}
	}
	// println!("\nmapping_for_f_is = {}", mapping_for_f_is);
	// println!("\nmapping_for_c_is = {}", mapping_for_c_is);

	// Update mapping for f
	mapping[ascii_to_usize('f')] = vec![mapping_for_f_is];
	mapping[ascii_to_usize('c')] = vec![mapping_for_c_is];

	// println!("\nMapping after sixth update:\n{:?}", mapping);

	// find last letter for mapping for g
	mapping.remove(ascii_to_usize('g'));

	let mut mapping: Vec<char> = mapping.iter().flatten().cloned().collect();
	let all_letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
	for letter in all_letters.iter() {
		if !mapping.iter().any(|&chr| chr == *letter) {
			mapping.push(*letter);
		}
	}

	// println!("\nMapping after seventh update:\n{:?}", mapping);

	mapping
}

fn get_differing_elements_in_vectors(five_segments: Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<char>) {

	// flatten the vector (15 elements, groups of five)
	let flat_vector: Vec<char> = five_segments.iter().flat_map(|string| string.iter()).cloned().collect();
	let mut flat_vector_clone: Vec<char> = flat_vector.clone();
	//println!("\nflat vector:\n{:?}", flat_vector);

	// Sort
	flat_vector_clone.sort();
	flat_vector_clone.dedup();
	// println!("\nflat vector_clone:\n{:?}", flat_vector_clone);

	// counter vector
	let mut counter: Vec<usize> = vec![0; flat_vector_clone.len()];

	for i in 0..flat_vector.len() {
		let idx: usize = flat_vector_clone.iter().position(|&chr| chr == flat_vector[i]).unwrap();
		counter[idx] += 1;
	}

	// println!("\ncounter:\n{:?}", counter);

	let mut number_two: Vec<char> = Vec::new();
	for i in 0..counter.len() {
		if counter[i] == 1 {
			number_two.push(flat_vector_clone[i]);
			counter.remove(i);
			flat_vector_clone.remove(i);
			break;
		}
	}

	// println!("\nflat vector_clone:\n{:?}", flat_vector_clone);
	// println!("\nnumber_two:\n{:?}", number_two);

	let mut number_five: Vec<char> = Vec::new();
	for i in 0..counter.len() {
		if counter[i] == 1 {
			number_five.push(flat_vector_clone[i]);
			counter.remove(i);
			flat_vector_clone.remove(i);
			break;
		}
	}
	// println!("\nnumber_five:\n{:?}", number_five);
	// println!("\ncounter:\n{:?}", counter);

	// these chars are in 3,2 and 3, 5.
	let mut chr_1: char = flat_vector_clone[0];
	for i in 0..counter.len() {
		if counter[i] == 2 {
			chr_1 = flat_vector_clone[i];
			counter.remove(i);
			flat_vector_clone.remove(i);
			break;
		}
	}

	let mut chr_2: char = flat_vector_clone[0];
	for i in 0..counter.len() {
		if counter[i] == 2 {
			chr_2 = flat_vector_clone[i];
			counter.remove(i);
			flat_vector_clone.remove(i);
			break;
		}
	}
	// println!("\nchr_1 = {}, chr_2 = {}", chr_1, chr_2);
	// println!("\nflat vector_clone:\n{:?}", flat_vector_clone);
	// println!("\nflat vector:\n{:?}", flat_vector);

	// Find number 3 and remove it from vector clone
	let mut idx_of_number_3: usize = 0;
	for number in five_segments.iter() {
		if number.iter().any(|&chr| chr == chr_1) && number.iter().any(|&chr| chr == chr_2) {
			break ;
		}
		idx_of_number_3 += 1;
	}

	// println!("\nidx_of_number_3 = {}\n", idx_of_number_3);

	let number_3: Vec<char> = five_segments[idx_of_number_3].clone();
	let mut new_five_segments: Vec<Vec<char>> = five_segments.clone();
	new_five_segments.remove(idx_of_number_3);
	let flat_vector: Vec<char> = new_five_segments.iter().flat_map(|string| string.iter()).cloned().collect();

	// println!("\nflat vector:\n{:?}", flat_vector);
	// find first chr_1 in one of the three five_segment numbers 
	let which_five_segment_idx_chr_1: usize = flat_vector.iter().position(|&chr| chr == chr_1).unwrap() / 5;
	// println!("\nwhich_five_segment_idx_chr_1 = {}\n", which_five_segment_idx_chr_1);

	// find first chr_2 in one of the three five_segment numbers 
	// let which_five_segment_idx_chr_2: usize = flat_vector.iter().position(|&chr| chr == chr_2).unwrap() / 5;
	// println!("\nwhich_five_segment_idx_chr_2 = {}\n", which_five_segment_idx_chr_2);

	// number one or number two in five_segments[which_five_element_idx]
	if new_five_segments[which_five_segment_idx_chr_1].iter().any(|&chr| chr == number_two[0]) {
		number_two.push(chr_1);
	} else {
		number_five.push(chr_1);
	}

	if number_two.len() == 2 {
		number_five.push(chr_2);
	} else {
		number_two.push(chr_2);
	}

	return (vec![number_two, number_five], number_3);

}

fn decode_one_input_line(parsed_data: &mut Vec<String>) -> usize {

	// println!("\nparsed_data:\n{:?}", parsed_data);

	// Get data before pipe sign, this is the encoding
	let encoded_data: Vec<Vec<char>> = get_all_before_pipe_sign(parsed_data[0].clone());
	// println!("\nencoded_data:\n{:?}", encoded_data);=

	// Create a mapping vector which maps encoded letters to original letters.
	let mut encoded_to_original_letters_mapping: Vec<Vec<char>> = vec![vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']; 7];
	// println!("\nencoded_to_original_letters_mapping:\n{:?}", encoded_to_original_letters_mapping);
	
	let mapping_line: Vec<char> = decode_one_line(&encoded_data, &mut encoded_to_original_letters_mapping);
	// println!("\nmapping_line = {:?}", mapping_line);

	let zero: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('b')],
								mapping_line[ascii_to_usize('c')],mapping_line[ascii_to_usize('e')],
								mapping_line[ascii_to_usize('f')], mapping_line[ascii_to_usize('g')]];
	let one: Vec<char> = vec![mapping_line[ascii_to_usize('c')], mapping_line[ascii_to_usize('f')]];
	let two: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('c')],
								mapping_line[ascii_to_usize('d')], mapping_line[ascii_to_usize('e')],
								mapping_line[ascii_to_usize('g')]];
	let three: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('c')],
								mapping_line[ascii_to_usize('d')], mapping_line[ascii_to_usize('f')],
								mapping_line[ascii_to_usize('g')]];
	let four: Vec<char> = vec![mapping_line[ascii_to_usize('b')], mapping_line[ascii_to_usize('d')],
								mapping_line[ascii_to_usize('c')], mapping_line[ascii_to_usize('f')]];
	let five: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('b')],
								mapping_line[ascii_to_usize('d')], mapping_line[ascii_to_usize('f')],
								mapping_line[ascii_to_usize('g')]];
	let six: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('b')],
								mapping_line[ascii_to_usize('d')], mapping_line[ascii_to_usize('e')],
								mapping_line[ascii_to_usize('f')], mapping_line[ascii_to_usize('g')]];
	let seven: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('c')],
								mapping_line[ascii_to_usize('f')]];
	let eight: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('b')],
								mapping_line[ascii_to_usize('c')], mapping_line[ascii_to_usize('d')],
								mapping_line[ascii_to_usize('e')], mapping_line[ascii_to_usize('f')],
								mapping_line[ascii_to_usize('g')]];
	let nine: Vec<char> = vec![mapping_line[ascii_to_usize('a')], mapping_line[ascii_to_usize('b')],
								mapping_line[ascii_to_usize('c')], mapping_line[ascii_to_usize('d')],
								mapping_line[ascii_to_usize('f')], mapping_line[ascii_to_usize('g')]];

	let numbers: Vec<Vec<char>> = vec![zero, one, two, three, four, five, six, seven, eight, nine];
	// println!("\nnumbers:\n{:?}", numbers);

	// Get data before after sign, this is the encoding
	let encoded_output: Vec<Vec<char>> = get_all_before_pipe_sign(parsed_data[1].clone());

	// println!("\nencoded_output: {:?}", encoded_output);

	// decode the output
	let decoded_output: usize = decode_output(encoded_output, numbers);

	decoded_output
}

fn compare_character_vectors(vector_1: Vec<char>, vector_2: Vec<char>) -> bool {

	// check lengths
	if vector_1.len() != vector_2.len() {
		return false;
	}

	let mut vector_1: Vec<char> = vector_1.clone();
	let mut vector_2: Vec<char> = vector_2.clone();

	// sort both vectors
	vector_1.sort();
	vector_2.sort();

	for i in 0..vector_1.len() {
		if vector_1[i] != vector_2[i] {
			return false;
		}
	}
	true
}

fn decode_output(encoded_output: Vec<Vec<char>>, numbers: Vec<Vec<char>>) -> usize {

	let mut number: Vec<usize> = vec![0; encoded_output.len()];
	for i in 0..encoded_output.len() {
		for j in 0..numbers.len() {
			if compare_character_vectors(encoded_output[i].clone(), numbers[j].clone()) {
				number[i] = j;
			}
		}
	}
	
	let mut result: usize = 0;
	for nb in number.iter() {
		result = result * 10 + nb;
	}
	result
}

pub fn day08(filename: &str) {

	let input_data: Vec<String> = get_data_from_input_file(filename);

	// Parse data
	let mut parsed_data: Vec<Vec<String>> = parse_data(input_data);
	let mut parsed_data_clone: Vec<Vec<String>> = parsed_data.clone();

	// Part1
	part1(&mut parsed_data); //because I pass a reference, parsed_data is not moved and I can use it afterwards. It is changed in the function though.

	// Part2
	let mut answer: usize = 0;
	for i in parsed_data_clone.iter_mut() {
		answer += decode_one_input_line(i);
	}
	println!("Day08 part2 answer = {}", answer);
}