// fn main() {

// 	let vector: Vec<String> = vec!["hilmi".to_string(), "yilmaz".to_string(), "codam".to_string()];

// 	println!("{:?}", vector);

// 	let mut iter = vector.iter();

// 	println!("{:?}", iter.next());
// 	println!("{:?}", iter.next().unwrap());

// 	let number: Vec<u32> = vec![1, 2, 3, 4, 5];
// 	println!("{:?}", number);
// 	let number: Vec<u32> = number.iter()
// 								.map(|nb| nb + 1)
// 								.map(|nb| nb * 10)
// 								.filter(|nb| nb > &25)
// 								.map(|nb| nb + 3)
// 								.collect();
// 	println!("{:?}", number);
// }

// #[derive(Debug)]
// struct Coordinate {
// 	x: u32,
// 	y: u32,
// }

// fn main() {

// 	let v: Vec<Coordinate> = (0..3)
// 								.map(|x| Coordinate {x: x + 10, y: x + 100,})
// 								.collect();

// 	println!("{:?}", v);
// }

// fn main() {

// 	let strings_vec: Vec<Vec<String>> = vec![vec!["hilmi".to_string(), "yilmaz".to_string(), "codam".to_string()]; 2];

// 	println!("{:?}", strings_vec);

// 	// let conversion: Vec<Vec<char>> = strings_vec.iter()
// 	// 												.map(|string| string.chars().collect())
// 	// 												.collect();

// 	let conversion: Vec<Vec<Vec<char>>> = strings_vec.iter()
// 													.map(|strings_vec| strings_vec.iter()
// 														.map(|string| string.chars().collect())
// 														.collect())
// 													.collect(); 

// 	println!("\n{:?}", conversion);

// }

// fn main() {

// 	let chars_vec: Vec<char> = vec!['c', 'd', 'e'];
// 	let	chars_to_stay: Vec<char> = vec!['c', 'd'];

// 	let result: bool = chars_to_stay.iter().any(|&c| c == 'd');

// 	println!("\n{:?}", chars_vec);

// 	println!("\n{:?}", chars_to_stay);


// 	let result: Vec<char> = chars_vec.iter().cloned().filter(|chr| chars_to_stay.iter().any(|&c| c == *chr)).collect();

// 	println!("result = {:?}", result);
// }

fn factorial(n: usize) -> usize {
	
	if n == 0 {
		return 1;
	}
	return n * factorial(n - 1);
}

fn main() {

	let n: usize = 20;

	let result: usize = factorial(n);

	println!("result = {}", result);

}