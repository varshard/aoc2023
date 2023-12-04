mod input;

use std::collections::HashSet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn read_line(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let input = line.split(':').collect::<Vec<&str>>();
    let card = input[1].split('|').collect::<Vec<&str>>();

    let mut win = HashSet::new();
    let mut have = vec![];

    for n in card[0].trim().split(' ') {
        if n == "" {
            continue;
        }
        win.insert(n.parse::<u32>().expect("not a number"));
    }

    for n in card[1].trim().split(' ') {
        if n == "" {
            continue;
        }
        have.push(n.parse::<u32>().unwrap());
    }
    
    (win, have)
}

fn sum_win(line: &str) -> u32 {
    let (win, have) = read_line(line);

    have.iter().fold(0, |sum, x| {
        match win.get(x) {
            None => {
                sum + 0
            }
            Some(_) => {
                return if sum > 1 { sum * 2} else { sum + 1};
            }
        }
    })
}

pub fn part1(lines: &str) -> u32 {
    lines.split('\n').fold(0, |sum, line | sum + sum_win(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line() {
        let (win, have) = read_line("lineCard 1: 41 48 83 86 17 | 83 86 6 31 17  9 48 53");

        let mut win_keys = win.into_iter().collect::<Vec<u32>>();
        win_keys.sort();
        assert_eq!(vec![17, 41,48,83,86], win_keys);
        assert_eq!(vec![83, 86, 6, 31, 17, 9, 48, 53], have);
    }

    #[test]
    fn test_sum_win() {
        assert_eq!(8, sum_win("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(18519, part1(input::INPUT));
    }
}
