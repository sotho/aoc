use regex::Regex;
use std::fs;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, PartialEq, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add for Cubes {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sum<Cubes> for Cubes {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Cubes>,
    {
        iter.fold(
            Cubes {
                ..Default::default()
            },
            |a, b| a + b,
        )
    }
}

impl Cubes {
    // example input: "3 blue, 4 red"
    fn get_cubes(value: &str) -> Cubes {
        value
            .split(", ")
            .map(|x| -> Cubes {
                let parts: Vec<&str> = x.split(' ').collect();
                assert_eq!(parts.len(), 2);
                let count = parts[0].parse::<u32>().unwrap();
                match parts[1] {
                    "red" => Cubes {
                        red: count,
                        ..Default::default()
                    },
                    "green" => Cubes {
                        green: count,
                        ..Default::default()
                    },
                    "blue" => Cubes {
                        blue: count,
                        ..Default::default()
                    },
                    _ => panic!("unknown color {}", parts[1]),
                }
            })
            .sum::<Cubes>()
    }

    fn max(self, other: Self) -> Self {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn get_max_cubes(value: &str) -> Cubes {
        value
            .split("; ")
            .map(|x| -> Cubes { Self::get_cubes(x) })
            .fold(
                Cubes {
                    ..Default::default()
                },
                |a, b| a.max(b),
            )
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn get_game(value: &str) -> (u32, Cubes) {
    let re = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();
    let caps = re.captures(value).unwrap();
    (
        caps[1].parse::<u32>().unwrap(),
        Cubes::get_max_cubes(&caps[2]),
    )
}

fn get_all_games(value: &str) -> Vec<(u32, Cubes)> {
    value.split_terminator("\n").map(|x| get_game(x)).collect()
}

fn get_valid_game_sum(value: &str) -> u32 {
    get_all_games(value)
        .iter()
        .filter(|(_id, cubes)| cubes.is_valid())
        .map(|(id, _cubes)| id)
        .sum()
}

fn main() {
    let input =
        fs::read_to_string("2.input").expect("Should have been able to read the file");

    let result = get_valid_game_sum(&input);

    println!("valid game sum: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_cubes() {
        assert_eq!(
            Cubes::get_cubes("3 blue, 4 red"),
            Cubes {
                red: 4,
                green: 0,
                blue: 3,
            }
        );
        assert_eq!(
            Cubes::get_cubes("8 green, 6 blue, 20 red"),
            Cubes {
                red: 20,
                green: 8,
                blue: 6,
            }
        );
    }

    #[test]
    fn test_max_cubes() {
        assert_eq!(
            Cubes::get_max_cubes(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Cubes {
                red: 20,
                green: 13,
                blue: 6,
            }
        );
    }

    #[test]
    fn test_game() {
        assert_eq!(
            get_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (
                1,
                Cubes {
                    red: 4,
                    green: 2,
                    blue: 6
                }
            )
        );
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(
            Cubes {
                red: 5,
                green: 5,
                blue: 5
            }
            .is_valid(),
            true
        );
        assert_eq!(
            Cubes {
                red: 13,
                green: 5,
                blue: 5
            }
            .is_valid(),
            false
        );
    }

    #[test]
    fn test_all_games() {
        let input =
            fs::read_to_string("2sample.input").expect("Should have been able to read the file");
        assert_eq!(
            get_all_games(&input),
            [
                (
                    1,
                    Cubes {
                        red: 4,
                        green: 2,
                        blue: 6
                    }
                ),
                (
                    2,
                    Cubes {
                        red: 1,
                        green: 3,
                        blue: 4
                    }
                ),
                (
                    3,
                    Cubes {
                        red: 20,
                        green: 13,
                        blue: 6
                    }
                ),
                (
                    4,
                    Cubes {
                        red: 14,
                        green: 3,
                        blue: 15
                    }
                ),
                (
                    5,
                    Cubes {
                        red: 6,
                        green: 3,
                        blue: 2
                    }
                )
            ]
        )
    }

    #[test]
    fn test_valid_game_sum() {
        let input =
            fs::read_to_string("2sample.input").expect("Should have been able to read the file");

        assert_eq!(get_valid_game_sum(&input), 8);
    }
}
