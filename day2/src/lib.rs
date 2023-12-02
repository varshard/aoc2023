mod input;

use std::collections::HashMap;

struct Day2 {
    cubes: HashMap<String, u32>
}

impl Day2 {
    fn new() -> Self {
        let mut cubes = HashMap::new();
        cubes.insert("red".to_string(), 12);
        cubes.insert("green".to_string(), 13);
        cubes.insert("blue".to_string(), 14);

        Day2{
            cubes
        }
    }
    fn read_line(&self, input: &str) -> Vec<HashMap<String,u32>> {
        let mut result = vec![];
        let rounds = input.split(';');

        for r in rounds {
            let mut map = HashMap::new();
            let cube_split = r.split(',').collect::<Vec<&str>>();
            for split in cube_split {
                let cube = split.trim().split(' ').collect::<Vec<&str>>();
                let num = cube[0].parse::<u32>().unwrap();
                let color = cube[1].to_string();

                match map.get(&color) {
                    None => {
                        map.insert(color, num);
                    }
                    Some(count) => {
                        map.insert(color, count + num);
                    }
                }
            }
            result.push(map);
        }

        result
    }

    fn part1_validate(&self, input: &str) -> (u32, bool) {
        let game = input.split(':').collect::<Vec<&str>>();
        let game_id = game[0].split(' ').collect::<Vec<&str>>();
        let id = game_id[1].parse::<u32>().unwrap();

        let grabs = self.read_line(game[1]);
        for grab in grabs {
            for (c, num) in grab {
                match self.cubes.get(&c) {
                    None => {
                        return (id, false);
                    }
                    Some(have) => {
                        if have < &num {
                            return (id, false);
                        }
                    }
                }
            }
        }
        (id, true)
    }

    fn part1(&self, input: String) -> u32 {
        input.split('\n').map(|line| self.part1_validate(line)).filter(|(_, valid)| *valid).fold(0, |sum, (id, _)| sum + id)
    }

    fn part2_line(&self, input: &str) -> u32 {
        let mut map: HashMap<String, u32> = HashMap::new();

        let game = input.split(':').collect::<Vec<&str>>();
        let grabs = self.read_line(game[1]);
        for grab in grabs {
            for (c, num) in grab {
                match map.get(&c) {
                    None => {
                        map.insert(c, num);
                    }
                    Some(count) => {
                        if &num > count {
                            map.insert(c, num);
                        }
                    }
                }
            }
        }

        map.iter().fold(1, |power, (_,v)| power * v)
    }
    fn part2(&self, input: &str) -> u32 {
        input.split('\n').map(|line| {
            self.part2_line(line)
        }).fold(0, |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_line() {
        let d = Day2::new();
        let read = d.read_line("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(read[0].get("blue"), Some(&3u32));
        assert_eq!(read[0].get("red"), Some(&4u32));
        assert_eq!(read[1].get("green"), Some(&2u32));
        assert_eq!(read[1].get("red"), Some(&1u32));
        assert_eq!(read[1].get("blue"), Some(&6u32));
        assert_eq!(read[2].get("green"), Some(&2u32));
    }

    #[test]
    fn validate_part1() {
        let d = Day2::new();
        assert_eq!(d.part1_validate("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), (1, true));
        assert_eq!(d.part1_validate("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), (3, false));

        assert_eq!(d.part1_validate("Game 100: 3 blue, 3 red, 6 green; 7 red, 2 green, 16 blue; 14 green, 9 red, 9 blue; 8 red, 10 green, 9 blue; 6 blue, 11 red"), (100, false));
    }

    #[test]
    fn part1_sample() {
        let d = Day2::new();
        assert_eq!(d.part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()), 8);
    }

    #[test]
    fn part1() {
        let d = Day2::new();
        assert_eq!(d.part1(input::INPUT.to_string()), 2156);
    }

    #[test]
    fn part2_sample() {
        let d = Day2::new();
        assert_eq!(d.part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(d.part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286);
    }

    #[test]
    fn part2() {
        let d = Day2::new();
        assert_eq!(d.part2(input::INPUT), 66909);
    }
}
