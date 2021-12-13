use crate::get_data_from_input_file;

fn parse_input_data(input_data: Vec<String>) -> Vec<Vec<u8>> {

	input_data.iter()
				.map(|string| string.chars()
					.map(|chr| chr.to_digit(10).unwrap() as u8)
					.collect())
				.collect()
}

// fn find_low_points_recursive(parsed_data: &mut Vec<Vec<u8>>, i: usize, j: usize, result: &mut Vec<u8>) {

// 	if i == parsed_data.len() && j == parsed_data[i].len() {
// 		return ;
// 	}
// 	// if we are not on the edges
// 	if i != 0 && i != parsed_data.len() - 1 && j != 0 && j != parsed_data[i].len() - 1 {
// 		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
// 			parsed_data[i][j] < parsed_data[i - 1][j] &&
// 			parsed_data[i][j] < parsed_data[i][j + 1] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 		else {
// 			find_low_points_recursive(parsed_data, i + 1, j, result);
// 			find_low_points_recursive(parsed_data, i, j + 1, result);
// 		}
// 	}
// 	else if i == 0 && j == 0 { // left top edge
// 		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
// 			parsed_data[i][j] < parsed_data[i][j + 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 		else {
// 			find_low_points_recursive(parsed_data, i + 1, j, result);
// 			find_low_points_recursive(parsed_data, i, j + 1, result);
// 		}
// 	}
// 	else if i == 0 && j == parsed_data[i].len() - 1 { // right top edge
// 		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
// 			parsed_data[i][j] < parsed_data[i + 1][j + 1] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 	}
// 	else if i == parsed_data.len() - 1 && j == 0 { // left bottom edge
// 		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
// 			parsed_data[i][j] < parsed_data[i - 1][j + 1] &&
// 			parsed_data[i][j] < parsed_data[i][j + 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 		else {
// 			find_low_points_recursive(parsed_data, i, j + 1, result);
// 		}
// 	}
// 	else if i == parsed_data.len() - 1 && j == parsed_data[i].len() - 1 { // right bottom edge
// 		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
// 			parsed_data[i][j] < parsed_data[i - 1][j - 1] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 	}
// 	else if i == 0 && j != 0 && j != parsed_data[i].len() - 1 {
// 		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
// 			parsed_data[i][j] < parsed_data[i][j + 1] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 	}
// 	else if i == parsed_data.len() - 1 && j != 0 && j != parsed_data[i].len() - 1 { // bottom row
// 		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
// 			parsed_data[i][j] < parsed_data[i][j + 1] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 		else {
// 			find_low_points_recursive(parsed_data, i, j + 1, result);
// 		}
// 	}
// 	else if i != 0 && i != parsed_data.len() - 1 && j == 0 {
// 		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
// 			parsed_data[i][j] < parsed_data[i + 1][j] &&
// 			parsed_data[i][j] < parsed_data[i][j + 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}
// 		else {
// 			find_low_points_recursive(parsed_data, i + 1, j, result);
// 			find_low_points_recursive(parsed_data, i, j + 1, result);
// 		}
// 	}
// 	else if i != 0 && i != parsed_data.len() - 1 && j == parsed_data[i].len() - 1 {
// 		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
// 			parsed_data[i][j] < parsed_data[i + 1][j] &&
// 			parsed_data[i][j] < parsed_data[i][j - 1]
// 		{
// 			result.push(parsed_data[i][j]);
// 		}	
// 	}

// }

fn low_point_handler(parsed_data: &Vec<Vec<u8>>, low_points: &mut Vec<(usize, usize)>, i: usize, j: usize) {
	// if we are not on the edges
	if i != 0 && i != parsed_data.len() - 1 && j != 0 && j != parsed_data[i].len() - 1 {
		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
			parsed_data[i][j] < parsed_data[i - 1][j] &&
			parsed_data[i][j] < parsed_data[i][j + 1] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == 0 && j == 0 { // left top edge
		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
			parsed_data[i][j] < parsed_data[i][j + 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == 0 && j == parsed_data[i].len() - 1 { // right top edge
		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
			parsed_data[i][j] < parsed_data[i + 1][j - 1] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == parsed_data.len() - 1 && j == 0 { // left bottom edge
		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
			parsed_data[i][j] < parsed_data[i - 1][j + 1] &&
			parsed_data[i][j] < parsed_data[i][j + 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == parsed_data.len() - 1 && j == parsed_data[i].len() - 1 { // right bottom edge
		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
			parsed_data[i][j] < parsed_data[i - 1][j - 1] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == 0 && j != 0 && j != parsed_data[i].len() - 1 {
		if 	parsed_data[i][j] < parsed_data[i + 1][j] && 
			parsed_data[i][j] < parsed_data[i][j + 1] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}
	}
	else if i == parsed_data.len() - 1 && j != 0 && j != parsed_data[i].len() - 1 { // bottom row
		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
			parsed_data[i][j] < parsed_data[i][j + 1] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}
	}
	else if i != 0 && i != parsed_data.len() - 1 && j == 0 {
		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
			parsed_data[i][j] < parsed_data[i + 1][j] &&
			parsed_data[i][j] < parsed_data[i][j + 1]
		{
			low_points.push((i,j));
		}
	}
	else if i != 0 && i != parsed_data.len() - 1 && j == parsed_data[i].len() - 1 {
		if 	parsed_data[i][j] < parsed_data[i - 1][j] && 
			parsed_data[i][j] < parsed_data[i + 1][j] &&
			parsed_data[i][j] < parsed_data[i][j - 1]
		{
			low_points.push((i,j));
		}	
	}
}

fn find_low_points_recursive(parsed_data: &Vec<Vec<u8>>, low_points: &mut Vec<(usize, usize)>, i: usize, j: usize) -> u8 {

	// if end of traversion (end of rows)
	if i == parsed_data.len() {
		return 1;
	}

	// if end of column
	if j == parsed_data[i].len() {
		return 0;
	}

	low_point_handler(&parsed_data, low_points, i, j);

	// traverse column
	if find_low_points_recursive(parsed_data, low_points, i, j + 1) == 1 {
		return 1;
	}

	// traverse rows
	return find_low_points_recursive(parsed_data, low_points, i + 1, 0);

}

fn flood_fill(parsed_data: &mut Vec<Vec<u8>>, x: i32, y: i32, counter: &mut usize) {
	
	if x < 0 || x as usize >= parsed_data.len() || y < 0 || y as usize >= parsed_data[x as usize].len() {
		return ;
	}

	if parsed_data[x as usize][y as usize] == 9 || parsed_data[x as usize][y as usize] == 10 {
		return ;
	} else {
		*counter += 1;
		parsed_data[x as usize][y as usize] = 10;
	}

	flood_fill(parsed_data, x + 1, y, counter);
	flood_fill(parsed_data, x - 1, y, counter);
	flood_fill(parsed_data, x, y + 1, counter);
	flood_fill(parsed_data, x, y - 1, counter);

}

pub fn day09(filename: &str) {

	let input_data: Vec<String> = get_data_from_input_file(filename);
	let mut parsed_data: Vec<Vec<u8>> = parse_input_data(input_data);
	let mut low_points_idx: Vec<(usize, usize)> = Vec::new();

	find_low_points_recursive(&parsed_data, &mut low_points_idx, 0, 0);

	let mut low_points_values: Vec<u8> = Vec::new();

	for idx in low_points_idx.iter() {
		low_points_values.push(parsed_data[idx.0][idx.1]);
	}

	let answer: u32 = low_points_values.iter()
									.map(|&nb| nb as u32 + 1)
									.sum();

	println!("Day09 part 1 answer = {}", answer);

	// For part 1, find area that is enclosed by 9's.
	let mut total_sum_basins: Vec<usize> = Vec::new();
	for coordinate in low_points_idx.iter() {
		let mut counter: usize = 0;
		flood_fill(&mut parsed_data, coordinate.0 as i32, coordinate.1 as i32, &mut counter);
		total_sum_basins.push(counter);
	}

	// Sort the array and only multipy last 3 elements.
	total_sum_basins.sort();
	let len: usize = total_sum_basins.len();
	println!("Day09 part 2 answer = {}", total_sum_basins[len - 1] * total_sum_basins[len - 2] * total_sum_basins[len - 3]);
}