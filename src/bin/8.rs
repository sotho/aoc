use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Map {
    instructions: Vec<bool>,
    map: HashMap<u16, (u16, u16)>,
    start: Vec<u16>,
    end: Vec<u16>,
}

impl Map {
    fn convert(value: &str) -> u16 {
        let mut factor = 1;
        assert_eq!(value.len(), 3);

        value
            .bytes()
            .rev()
            .map(|x| {
                assert!((x >= 'A' as u8 && x <= 'Z' as u8) || (x >= '0' as u8 && x <= '9' as u8));
                let result = if x >= 'A' as u8 {
                    (x - 'A' as u8) as u16 * factor
                } else {
                    (x - '0' as u8 + 26) as u16 * factor
                };
                factor *= 36;

                result
            })
            .sum()
    }

    fn new(value: &str, is_ghost: bool) -> Self {
        let mut result: Map = Map {
            instructions: Vec::new(),
            map: HashMap::new(),
            start: vec![],
            end: vec![],
        };

        let re = Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();

        value
            .split_terminator('\n')
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                if !line.contains("=") {
                    line.chars().for_each(|c| {
                        result.instructions.push(match c {
                            'L' => false,
                            'R' => true,
                            _ => panic!(),
                        })
                    });
                } else {
                    let caps = re.captures(line).unwrap();
                    assert_eq!(
                        result.map.insert(
                            Self::convert(&caps[1]),
                            (Self::convert(&caps[2]), Self::convert(&caps[3]))
                        ),
                        None
                    );
                }
            });

        if is_ghost {
            result
                .map
                .keys()
                .filter(|p| **p % 36 == 0)
                .for_each(|p| result.start.push(*p));
            result.start.sort();
            result
                .map
                .keys()
                .filter(|p| **p % 36 == 25)
                .for_each(|p| result.end.push(*p));
            result.end.sort();
        } else {
            result.start.push(Self::convert("AAA"));
            result.end.push(Self::convert("ZZZ"));
        }

        result
    }

    fn step(&self, position: &u16, direction: &bool) -> u16 {
        match self.map.get(position) {
            Some(paths) => match direction {
                false => paths.0,
                true => paths.1,
            },
            None => panic!(),
        }
    }

    // walk from start to end and count steps
    fn walk(&self) -> usize {
        let mut pos = self.start.clone();
        let mut count = 0;

        let mut best_endpost_count = 0;
        for direction in self.instructions.iter().cycle() {
            let endpos_count = pos.iter().map(|p| self.end.contains(p)).filter(|x| *x).count();
            if count % 10000000 == 0 || endpos_count > best_endpost_count {
                println!("pos: {:?}, count: {}, end count: {}", pos, count, endpos_count);
                best_endpost_count = endpos_count.max(best_endpost_count);
            }
            if pos.iter().map(|p| self.end.contains(p)).all(|x| x) {
                break;
            }

            pos = pos.iter().map(|p| self.step(p, direction)).collect();
            count += 1;
        }

        count
    }
}

fn main() {
    let input = fs::read_to_string("8.input").expect("Should have been able to read the file");

    let map = Map::new(&input, false);
    println!("steps: {}", map.walk());

    let ghost = Map::new(&input, true);
    println!("ghost steps: {}", ghost.walk()); // 15726453850399
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_maps() -> (Map, Map, Map) {
        let input1 =
            fs::read_to_string("8sample1.input").expect("Should have been able to read the file");
        let input2 =
            fs::read_to_string("8sample2.input").expect("Should have been able to read the file");
        let input3 =
            fs::read_to_string("8sample3.input").expect("Should have been able to read the file");

        (
            Map::new(&input1, false),
            Map::new(&input2, false),
            Map::new(&input3, true),
        )
    }

    #[test]
    fn test_map_new() {
        let (map1, map2, ghost) = get_maps();

        assert_eq!(map1.instructions, [true, false]);
        assert_eq!(map2.instructions, [false, false, true]);
        assert_eq!(ghost.instructions, [false, true]);
        assert_eq!(
            map1.map.get(&Map::convert("AAA")),
            Some(&(Map::convert("BBB"), Map::convert("CCC")))
        );
        assert_eq!(
            map2.map.get(&Map::convert("AAA")),
            Some(&(Map::convert("BBB"), Map::convert("BBB")))
        );
        assert_eq!(
            ghost.map.get(&Map::convert("11A")),
            Some(&(Map::convert("11B"), Map::convert("XXX")))
        );
        assert_eq!(map1.start, [Map::convert("AAA")]);
        assert_eq!(map1.end, [Map::convert("ZZZ")]);
        assert_eq!(ghost.start, [Map::convert("11A"), Map::convert("22A")]);
        assert_eq!(ghost.end, [Map::convert("11Z"), Map::convert("22Z")]);
    }

    #[test]
    fn test_convert() {
        let f = 36;
        assert_eq!(Map::convert("AAA"), 0);
        assert_eq!(Map::convert("BBB"), 1 + f + f * f);
        assert_eq!(Map::convert("BCD"), 3 + 2 * f + 1 * f * f);
        assert_eq!(Map::convert("ZZZ"), 25 + 25 * f + 25 * f * f);
        assert_eq!(Map::convert("11A"), 0 + 27 * f + 27 * f * f);
    }

    #[test]
    fn test_step() {
        let (map1, map2, ghost) = get_maps();

        assert_eq!(map1.step(&Map::convert("AAA"), &false), Map::convert("BBB"));
        assert_eq!(map1.step(&Map::convert("AAA"), &true), Map::convert("CCC"));
        assert_eq!(map2.step(&Map::convert("AAA"), &false), Map::convert("BBB"));
        assert_eq!(map2.step(&Map::convert("AAA"), &true), Map::convert("BBB"));
        assert_eq!(ghost.step(&Map::convert("11A"), &false), Map::convert("11B"));
        assert_eq!(ghost.step(&Map::convert("22A"), &false), Map::convert("22B"));
    }

    #[test]
    fn test_walk() {
        let (map1, map2, ghost) = get_maps();

        assert_eq!(map1.walk(), 2);
        assert_eq!(map2.walk(), 6);
        assert_eq!(ghost.walk(), 6);
    }

    #[test]
    fn test_big_map() {
        let input = fs::read_to_string("8.input").expect("Should have been able to read the file");
        let ghost = Map::new(&input, true);

        assert_eq!(
            ghost.start,
            [
                Map::convert("AAA"),
                Map::convert("BPA"),
                Map::convert("BVA"),
                Map::convert("FDA"),
                Map::convert("NDA"),
                Map::convert("QCA"),
            ]
        );
        assert_eq!(
            ghost.end,
            [
                Map::convert("HJZ"),
                Map::convert("PQZ"),
                Map::convert("RFZ"),
                Map::convert("SBZ"),
                Map::convert("VPZ"),
                Map::convert("ZZZ"),
            ]
        );
    }
}
