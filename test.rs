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

#[derive(Debug)]
struct Coordinate {
	x: u32,
	y: u32,
}

fn main() {

	let v: Vec<Coordinate> = (0..3)
								.map(|x| Coordinate {x: x + 10, y: x + 100,})
								.collect();

	println!("{:?}", v);
}