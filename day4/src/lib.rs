mod input;

use std::collections::{HashMap, HashSet};

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

fn count_win(line: &str) -> u32 {
    let (win, have) = read_line(line);

    have.iter().fold(0, |sum, x| {
        match win.get(x) {
            None => {
                sum + 0
            }
            Some(_) => {
                sum + 1
            }
        }
    })
}

pub fn part1(lines: &str) -> u32 {
    lines.split('\n').fold(0, |sum, line | sum + sum_win(line))
}

pub fn part2(input: &str) -> u32 {
    let mut cards = HashMap::new();
    let mut count = 0;
    for (i, line) in input.split('\n').enumerate() {
        cards.insert((i+ 1) as u32, count_win(line));
    }


    for (k, _) in input.split('\n').enumerate() {
        count += process_copy(&((k+1) as u32), &cards);
    }

    count + cards.len() as u32
}

fn process_copy(id: &u32, cards: &HashMap<u32, u32>) -> u32 {
    let mut count = 0;
    match cards.get(&id) {
        None => {}
        Some(v) => {
            count += v;
            for i in *id..*id+v+1 {
                if cards.contains_key(&(i + 1)) {
                    count += process_copy(&(i + 1), cards);
                }
            }
        }
    }
    count
}

pub fn part2v2(input: &str) -> u32 {
    let mut cards = HashMap::new();
    let mut mem = HashMap::new();
    for (i, line) in input.split('\n').enumerate() {
        cards.insert((i+ 1) as u32, count_win(line));
        mem.insert((i+1) as u32, 1);
    }

    process_copy2(&cards, &mut mem);

    let mut count = 0;
    for i in 1..cards.len()+1 {
        count += mem.get(&(i as u32)).unwrap();
    }
    count
}

fn process_copy2(cards: &HashMap<u32, u32>, mut mem: &mut HashMap<u32, u32>) {
    for i in (1..cards.len()).rev() {
        let key = &((i) as u32);
        let win = cards.get(key).unwrap();
        let mut count = 0;

        for j in i+1..(*win as usize + i + 1) {
            match mem.get(&(j as u32)) {
                None => {}
                Some(c) => {
                    count += c;
                    // mem.insert((i - 1) as u32,  c + mem.get(key).unwrap());
                }
            }
        }
        mem.insert(i as u32, mem.get(key).unwrap() + count);
    }
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

    #[test]
    fn test_process_copy_sample1() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 1);
        map.insert(3, 0);
        assert_eq!(3, process_copy(&1, &map));
    }

    #[test]
    fn test_process_copy_sample2() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 2);
        map.insert(3, 1);
        map.insert(4, 0);
        assert_eq!(6, process_copy(&1, &map));
    }

    #[test]
    fn test_process_copy2_sample1() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 1);
        map.insert(3, 0);

        let mut mem: HashMap<u32,u32> = HashMap::new();
        mem.insert(1, 1);
        mem.insert(2, 1);
        mem.insert(3, 1);

        process_copy2(&map, &mut mem);
        assert_eq!(&4, mem.get(&1).unwrap());
    }

    #[test]
    fn test_process_copy2_sample2() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 2);
        map.insert(3, 1);
        map.insert(4, 0);

        let mut mem: HashMap<u32,u32> = HashMap::new();
        mem.insert(1, 1);
        mem.insert(2, 1);
        mem.insert(3, 1);
        mem.insert(4, 1);

        process_copy2(&map, &mut mem);
        assert_eq!(&7, mem.get(&1).unwrap());
    }

    #[test]
    fn test_process_copy2_sample3() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        map.insert(1, 4);
        map.insert(2, 2);
        map.insert(3, 2);
        map.insert(4, 1);
        map.insert(5, 0);
        map.insert(6, 0);

        let mut mem: HashMap<u32,u32> = HashMap::new();
        mem.insert(1, 1);
        mem.insert(2, 1);
        mem.insert(3, 1);
        mem.insert(4, 1);
        mem.insert(5, 1);
        mem.insert(6, 1);

        process_copy2(&map, &mut mem);
        assert_eq!(&15, mem.get(&1).unwrap());
    }

    #[test]
    fn test_count_win() {
        assert_eq!(0, count_win("Card 198: 87  3 64 10 88 45 16 40 23 60 | 63 77 36 52 47 76 84 96 19 13 73 39 26 93 21 22  7 15 95 30 33 89 28 20 50"));
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(30, part2v2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
    }

    #[test]
    fn part2_input() {
        print!("{}", part2v2(input::INPUT));
    }
}
