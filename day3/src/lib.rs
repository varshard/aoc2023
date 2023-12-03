use std::collections::HashSet;
use std::ops::Range;

mod input;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

pub struct RangeNum {
	range: Range<usize>,
	value: u64,
}

pub fn read_line(line: &str) -> (Vec<RangeNum>, HashSet<usize>) {
	let mut range = vec![];
	let mut symbols = HashSet::new();
	let matrix = line.chars().collect::<Vec<char>>();
	let mut digit_start: i32 = -1;
	for i in 0..matrix.len() {
		if matrix[i].is_digit(10) {
			if digit_start < 0 {
				digit_start = i as i32;
			}
			if i == matrix.len() - 1 {
				let digit = line[digit_start as usize..matrix.len()].to_string().parse::<u64>().unwrap();
				range.push(RangeNum {
					range: (digit_start as usize..matrix.len()),
					value: digit,
        });
			}
		} else {
			if digit_start > -1 {
				let digit = line[digit_start as usize..i].to_string().parse::<u64>().unwrap();
				range.push(RangeNum {
					range: (digit_start as usize..i),
					value: digit,
        });
				digit_start = -1;
			}

			if matrix[i] != '.' {
				symbols.insert(i);
			}
		}
	}

	(range, symbols)
}

pub fn part1(lines: &str) -> u64 {
	let mut sum = 0;
	let mut ranges = vec![];
	let mut symbols = vec![];

	for line in lines.split('\n') {
		let (range, s) = read_line(line);
		ranges.push(range);
		symbols.push(s);
	}

	for (i, l) in symbols.iter().enumerate() {
		for symbol_idx in l.iter() {
			for r in &ranges[i] {
				if r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
					sum += r.value;
				}
			}

			if i + 1 < ranges.len() {
				for r in &ranges[i + 1] {
					if r.range.contains(&(symbol_idx)) || r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
						sum += r.value;
					}
				}
			}

			if i > 0 {
				for r in &ranges[i - 1] {
					if r.range.contains(&(symbol_idx)) || r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
						sum += r.value;
					}
				}
			}
		}
	}

	sum
}

pub fn part2(lines: &str) -> u64 {
	let mut sum:u64 = 0;
	let mut ranges = vec![];
	let mut symbols = vec![];

	for line in lines.split('\n') {
		let (range, s) = read_line(line);
		ranges.push(range);
		symbols.push(s);
	}

	for (i, l) in symbols.iter().enumerate() {
		for symbol_idx in l.iter() {
			let mut product = vec![];
			if i > 0 {
				for r in &ranges[i - 1] {
					if r.range.contains(&(symbol_idx)) || r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
						product.push(r.value);
					}
				}
			}

			for r in &ranges[i] {
				if r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
					product.push(r.value);
				}
			}

			if i + 1 < ranges.len() {
				for r in &ranges[i + 1] {
					if r.range.contains(&(symbol_idx)) || r.range.contains(&(symbol_idx - 1)) || r.range.contains(&(symbol_idx + 1)) {
						product.push(r.value);
					}
				}
			}

			if product.len() > 1 {
				sum += product.iter().fold(1, |accum, x| accum * x);
			}
		}
	}

	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_read_line() {
		let (line, symbols) = &read_line("467.*114");
		assert_eq!(line[0].range, (0..3));
		assert_eq!(line[0].value, 467);

		assert_eq!(line[1].range, (5..8));
		assert_eq!(line[1].value, 114);

		assert!(symbols.contains(&4));
	}

	#[test]
	fn test_part1() {
		assert_eq!(35, part1("...*......
..35..633."));
		assert_eq!(58, part1(".58*......"));
		assert_eq!(58, part1("...*58...."));

		assert_eq!(467, part1("467..114..
...*......"));

		assert_eq!(4361, part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."));
	}

	#[test]
	fn test_part1_full() {
		assert_eq!(514969, part1(input::INPUT))
	}

	#[test]
	fn test_part2_sample() {
		assert_eq!(16345, part2("467..114..
...*......
..35..633."));

		assert_eq!(451490, part2("......755.
...$.*....
.664.598.."));

		assert_eq!(467835, part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."));
	}

	#[test]
	fn test_part2_full() {
		// assert_eq!(514969, part1(input::INPUT))
		print!("{}", part2(input::INPUT));
	}
}

