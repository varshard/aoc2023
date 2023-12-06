mod tree;
mod input;

use std::ops::Range;
use tree::{Node};

fn read_seed(line: &str) -> Vec<u64> {
    let splitted = line.split(':').collect::<Vec<&str>>();

    let seeds = splitted[1].trim().split(' ').collect::<Vec<&str>>();
    seeds.iter().map(|s| {
        s.parse::<u64>().unwrap()
    }).collect()
}

fn read_seed_range(line: &str) -> Vec<Range<u64>> {
    let splitted = line.split(':').collect::<Vec<&str>>();

    let seeds = splitted[1].trim().split(' ').collect::<Vec<&str>>();
    let nums = seeds.iter().map(|s| {
        s.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();

    let mut ranges = vec![];

    for (i, n ) in nums.iter().enumerate() {
        if i %2 == 0 {
            ranges.push(*n..n+nums[i+1])
        }
    }
    ranges
}

struct Almanac  {
    seeds: Vec<u64>,
    seed_range: Vec<Range<u64>>,
    seed_to_soil: Option<Node>,
    soil_to_fert: Option<Node>,
    fert_to_water: Option<Node>,
    water_to_light: Option<Node>,
    light_to_temp: Option<Node>,
    temp_to_humid: Option<Node>,
    humid_to_loc: Option<Node>,
    soil_to_seed: Option<Node>,
    fert_to_soil: Option<Node>,
    water_to_fert: Option<Node>,
    light_to_water: Option<Node>,
    temp_to_light: Option<Node>,
    humid_to_temp: Option<Node>,
    loc_to_humid: Option<Node>,
    loc_ranges: Vec<Range<u64>>
}

impl Almanac {
    pub fn find_loc(&self, seed: &u64) -> u64 {
        let soil = self.seed_to_soil.as_ref().unwrap().find(seed);
        let fertilizer = self.soil_to_fert.as_ref().unwrap().find(&soil);
        let water = self.fert_to_water.as_ref().unwrap().find(&fertilizer);
        let light = self.water_to_light.as_ref().unwrap().find(&water);
        let temp = self.light_to_temp.as_ref().unwrap().find(&light);
        let humid = self.temp_to_humid.as_ref().unwrap().find(&temp);
        self.humid_to_loc.as_ref().unwrap().find(&humid)
    }

    pub fn find_seed(&self, loc: &u64) -> u64 {
        let humid = self.loc_to_humid.as_ref().unwrap().find(&loc);
        let temp = self.humid_to_temp.as_ref().unwrap().find(&humid);
        let light = self.temp_to_light.as_ref().unwrap().find(&temp);
        let water = self.light_to_water.as_ref().unwrap().find(&light);
        let fert = self.water_to_fert.as_ref().unwrap().find(&water);
        let soil = self.fert_to_soil.as_ref().unwrap().find(&fert);
        self.soil_to_seed.as_ref().unwrap().find(&soil)
    }
}

fn read_input(input: &str) -> Almanac{
    let mut lines = input.split('\n').peekable();
    let seed_line = lines.next().unwrap();
    let mut almanac = Almanac{
        seeds: read_seed(seed_line),
        seed_range: read_seed_range(seed_line),
        seed_to_soil: None,
        soil_to_fert: None,
        fert_to_water: None,
        water_to_light: None,
        light_to_temp: None,
        temp_to_humid: None,
        humid_to_loc: None,
        soil_to_seed: None,
        fert_to_soil: None,
        water_to_fert: None,
        light_to_water: None,
        temp_to_light: None,
        humid_to_temp: None,
        loc_to_humid: None,
        loc_ranges: vec![]
    };

    while let Some(l) = lines.next() {
        if l == "seed-to-soil map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (dst, src) = read_to_range(l);
        let n = Node::new(dst.clone(), src.clone());
        match almanac.seed_to_soil {
            None => {
                almanac.seed_to_soil = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
        let n = Node::new(dst.clone(), src.clone());
        match almanac.soil_to_seed {
            None => {
                almanac.soil_to_seed = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }

    while let Some(l) = lines.next() {
        if l == "soil-to-fertilizer map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (src, dst) = read_to_range(l);
        let n = Node::new(src.clone(), dst.clone());
        match almanac.soil_to_fert {
            None => {
                almanac.soil_to_fert = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
        let n = Node::new(dst.clone(), src.clone());
        match almanac.fert_to_soil {
            None => {
                almanac.fert_to_soil = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }

    while let Some(l) = lines.next() {
        if l == "fertilizer-to-water map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (src, dst) = read_to_range(l);
        let n = Node::new(src.clone(), dst.clone());
        match almanac.fert_to_water {
            None => {
                almanac.fert_to_water = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }

        let n = Node::new(dst.clone(), src.clone());
        match almanac.water_to_fert {
            None => {
                almanac.water_to_fert = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }


    while let Some(l) = lines.next() {
        if l == "water-to-light map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (src, dst) = read_to_range(l);
        let n = Node::new(src.clone(), dst.clone());
        match almanac.water_to_light {
            None => {
                almanac.water_to_light = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }

        let reverse = Node::new(dst.clone(), src.clone());
        match almanac.light_to_water {
            None => {
                almanac.light_to_water = Some(reverse);
            }
            Some(ref mut tree) => {
                tree.insert(reverse);
            }
        }
    }

    while let Some(l) = lines.next() {
        if l == "light-to-temperature map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }

        let (src, dst) = read_to_range(l);
        let n = Node::new(src.clone(), dst.clone());
        match almanac.light_to_temp {
            None => {
                almanac.light_to_temp = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }

        let n = Node::new(dst.clone(), src.clone());
        match almanac.temp_to_light {
            None => {
                almanac.temp_to_light = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }

    while let Some(l) = lines.next() {
        if l == "temperature-to-humidity map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }

        let (src, dst) = read_to_range(l);
        let n = Node::new(src.clone(), dst.clone());
        match almanac.temp_to_humid {
            None => {
                almanac.temp_to_humid = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }

        let n = Node::new(dst.clone(), src.clone());
        match almanac.humid_to_temp {
            None => {
                almanac.humid_to_temp = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }

    while let Some(l) = lines.next() {
        if l == "humidity-to-location map:" {
            break;
        }
    }

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (src, dst) = read_to_range(l);
        almanac.loc_ranges.push(src.clone());
        let n = Node::new(src.clone(), dst.clone());
        match almanac.humid_to_loc {
            None => {
                almanac.humid_to_loc = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
        let n = Node::new(dst.clone(), src.clone());
        match almanac.loc_to_humid {
            None => {
                almanac.loc_to_humid = Some(n);
            }
            Some(ref mut tree) => {
                tree.insert(n);
            }
        }
    }

    almanac
}

fn read_to_range(l: &str) -> (Range<u64>, Range<u64>){
    let col = l.split(' ').collect::<Vec<&str>>();

    let ranges = col.iter().map(|s| {
        s.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();

    let len = ranges[2];
    let dst = ranges[0];
    let src = ranges[1];

    (dst..dst+len, src..src+len)
}

fn part1(input: &str) -> u64 {
    let almanac = &read_input(input);
    almanac.seeds.iter().fold(u64::MAX, |loc, seed| {
        std::cmp::min(loc, almanac.find_loc(seed))
    })
}

fn part2(input: &str) -> u64 {
    // TODO: correct this
    let almanac = &read_input(input);
    // almanac.loc_ranges.iter().fold(u64::MAX, |min_loc, range| {
    //     let seed= almanac.find_seed(&range.start);
    //     almanac.seed_range.clone().iter().fold(min_loc, |loc, seed_range| {
    //         if seed_range.contains(&seed) {
    //             std::cmp::min(loc, min_loc)
    //         } else {
    //             min_loc
    //         }
    //     })
    // })

    let mut min_loc = u64::MAX;
    for r in almanac.seed_range.iter() {
        for s in r.start..r.end {
            min_loc = std::cmp::min(min_loc, almanac.find_loc(&s));
        }
    }
    min_loc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_seeds() {
        assert_eq!(vec![79, 14, 55, 13], read_seed("seeds: 79 14 55 13"));
    }

    #[test]
    fn it_read_seed_ranges() {
        assert_eq!(vec![79..79+14, 55..55+13], read_seed_range("seeds: 79 14 55 13"));
    }

    #[test]
    fn it_read_to_ranges() {
        let (src, dst) = read_to_range("50 92 2");
        assert_eq!(50..52, dst);
        assert_eq!(92..94, src);
    }

    #[test]
    fn it_read_input() {
        let almanac = &read_input(input::SAMPLE);
        assert_eq!(vec![79,14,55,13], almanac.seeds);
        assert_eq!(53, almanac.seed_to_soil.as_ref().unwrap().find(&51));

        assert_eq!(82u64, almanac.find_loc(&79u64));
        assert_eq!(43u64, almanac.find_loc(&14u64));
        assert_eq!(86u64, almanac.find_loc(&55u64));
        assert_eq!(35u64, almanac.find_loc(&13u64));
    }

    #[test]
    fn  part1_work() {
        assert_eq!(35, part1(input::SAMPLE));
    }

    #[test]
    fn it_solve_part1() {
        assert_eq!(265018614u64, part1(input::INPUT));
    }

    #[test]
    fn it_solve_part2_sample() {
        assert_eq!(46, part2(input::SAMPLE));
    }
    #[test]
    fn it_solve_part2() {
        print!("{}", part2(input::INPUT));
    }
}
