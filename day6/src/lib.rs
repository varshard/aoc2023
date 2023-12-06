fn read_input(input: &str) -> (Vec<u64>, Vec<u64>) {
  let lines = input.split('\n').collect::<Vec<&str>>();
  let times = lines[0].strip_prefix("Time:").unwrap().trim().split_whitespace().collect::<Vec<&str>>().iter().map(|n| {
    n.trim().parse::<u64>().unwrap()
  }).collect::<Vec<u64>>();

	let distances = lines[1].strip_prefix("Distance:").unwrap().trim().split_whitespace().collect::<Vec<&str>>().iter().map(|n| {
		n.trim().parse::<u64>().unwrap()
	}).collect::<Vec<u64>>();

	(times, distances)
}

fn part1(input: &str) -> u64{
	let (times, distances) = read_input(input);
	let mut total = 0;

	for i in 0..times.len() {
		let mut valid = 0;
		let time = times[i];
		let distance = distances[i];

		for j in 0..=time {
			let dt = time - j;
			if dt * j > distance {
				valid += 1;
			}
		}
		if total == 0 {
			total = valid;
		} else {
			total *= valid;
		}
	}
	total
}

fn part2(input: &str) -> u64 {
	let lines = input.split('\n').collect::<Vec<&str>>();
	let times = lines[0].strip_prefix("Time:").unwrap().trim().split_whitespace().collect::<Vec<&str>>();
	let time = times.join("").parse::<u64>().unwrap();

	let distances = lines[1].strip_prefix("Distance:").unwrap().trim().split_whitespace().collect::<Vec<&str>>();
	let distance = distances.join("").parse::<u64>().unwrap();

	let mut valid = 0;
	for j in 0..=time {
		let dt = time - j;
		if dt * j > distance {
			valid += 1;
		}
	}
	valid
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let (times, distances) = read_input("Time:      7  15   30
Distance:  9  40  200");
		assert_eq!(3, times.len());
		assert_eq!(3, distances.len());
	}

	#[test]
	fn test_part1() {
		assert_eq!(9, part1("Time: 30
Distance: 200"));
		assert_eq!(288, part1("Time:      7  15   30
Distance:  9  40  200"));
	}

	#[test]
	fn test_part1_input() {
		print!("{}", part1("Time:        45     97     72     95
Distance:   305   1062   1110   1695"))
	}

	#[test]
	fn test_part2() {
		print!("{}", part2("Time:        45     97     72     95
Distance:   305   1062   1110   1695"))
	}
}
