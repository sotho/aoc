use std::fs;

#[derive(Debug, PartialEq)]
struct MapEntry {
    source: u64,
    dest: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new(value: &str) -> Self {
        let mut seeds = vec![];
        let mut maps: Vec<Map> = vec![];
        let mut index: i32 = -1;

        for line in value.split_terminator('\n').filter(|line| !line.is_empty()) {
            if line.starts_with("seeds: ") {
                seeds = line[6..]
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            } else if line.ends_with("map:") {
                index += 1;
                maps.push(Map { entries: vec![] });
            } else {
                let mut iter = line.split(' ').filter(|x| !x.is_empty());
                let Some(dest) = iter.next() else {
                    panic!();
                };
                let Some(source) = iter.next() else {
                    panic!();
                };
                let Some(length) = iter.next() else {
                    panic!();
                };
                assert_eq!(iter.next(), None);

                maps[index as usize].entries.push(MapEntry {
                    source: source.parse::<u64>().unwrap(),
                    dest: dest.parse::<u64>().unwrap(),
                    length: length.parse::<u64>().unwrap(),
                });
            }
        }

        Almanac {
            seeds: seeds,
            maps: maps,
        }
    }

    fn map(&self, map_index: usize, value: u64) -> u64 {
        match self.maps[map_index].entries.iter().find_map(|map_entry| {
            if value >= map_entry.source && value < map_entry.source + map_entry.length {
                Some(value - map_entry.source + map_entry.dest)
            } else {
                None
            }
        }) {
            Some(mapped_value) => mapped_value,
            None => value,
        }
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let mut result = seed;
        (0..self.maps.len()).for_each(|index| result = self.map(index, result));
        result
    }

    fn get_min_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.seed_to_location(*seed))
            .min()
            .unwrap()
    }

    fn get_min_location_seed_range(&self) -> u64 {
        (0..self.seeds.len())
            .step_by(2)
            .inspect(|index| println!("at seed {}", self.seeds[*index]))
            .map(|index| {
                (self.seeds[index]..self.seeds[index] + self.seeds[index + 1])
                    .map(|seed| self.seed_to_location(seed))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("5.input").expect("Should have been able to read the file");
    let almanac = Almanac::new(&input);

    println!("lowest location: {}", almanac.get_min_location());
    println!("lowest location range: {}", almanac.get_min_location_seed_range());
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_almanac() -> Almanac {
        let input =
            fs::read_to_string("5sample.input").expect("Should have been able to read the file");

        Almanac::new(&input)
    }

    #[test]
    fn test_new() {
        let almanac = get_almanac();

        assert_eq!(almanac.seeds, [79, 14, 55, 13]);
        assert_eq!(
            almanac.maps[0].entries[0],
            MapEntry {
                source: 98,
                dest: 50,
                length: 2,
            }
        );
        assert_eq!(
            almanac.maps[6].entries[1],
            MapEntry {
                source: 93,
                dest: 56,
                length: 4,
            }
        );
    }

    #[test]
    fn test_map() {
        let almanac = get_almanac();

        assert_eq!(almanac.map(0, 0), 0);
        assert_eq!(almanac.map(0, 49), 49);
        assert_eq!(almanac.map(0, 50), 52);
        assert_eq!(almanac.map(0, 97), 99);
        assert_eq!(almanac.map(0, 98), 50);
        assert_eq!(almanac.map(0, 99), 51);
        assert_eq!(almanac.map(0, 100), 100);
    }

    #[test]
    fn test_seed_to_location() {
        let almanac = get_almanac();

        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(55), 86);
        assert_eq!(almanac.seed_to_location(13), 35);
    }

    #[test]
    fn test_min_location() {
        let almanac = get_almanac();

        assert_eq!(almanac.get_min_location(), 35);
    }
    #[test]
    fn test_min_location_range() {
        let almanac = get_almanac();

        assert_eq!(almanac.get_min_location_seed_range(), 46);
    }
}
