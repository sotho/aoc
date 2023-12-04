use std::fs;

#[derive(Debug, PartialEq)]
struct Schema {
    field: Vec<Vec<char>>,
}

impl Schema {
    fn new(value: &str) -> Self {
        Schema {
            field: value
                .split_terminator('\n')
                .map(|line| -> Vec<char> {
                    let mut result: Vec<char> = Vec::new();
                    result.extend(line.chars());
                    result
                })
                .collect::<Vec<Vec<char>>>(),
        }
    }

    fn len_x(&self) -> usize {
        if self.len_y() == 0 {
            0
        } else {
            self.field[0].len()
        }
    }

    fn len_y(&self) -> usize {
        self.field.len()
    }

    fn get(&self, x: i32, y: i32) -> char {
        if x >= 0 && y >= 0 && (y as usize) < self.len_y() && (x as usize) < self.len_x() {
            self.field[y as usize][x as usize]
        } else {
            '.' // is not a number nor a symbol
        }
    }

    // find all numbers that might be a serial
    fn find_serial(&self) -> SerialIterator {
        SerialIterator {
            schema: self,
            pos: SerialPosition {
                x: 0,
                y: 0,
                length: 0,
            },
        }
    }

    // find all adjacent chars of a number
    fn find_adjacent<'a>(&'a self, pos: &'a SerialPosition) -> AdjacentIterator {
        AdjacentIterator {
            schema: self,
            pos: pos,
            x: -2,
            y: -2,
        }
    }

    // check if really serial
    fn is_serial(&self, pos: &SerialPosition) -> bool {
        self.find_adjacent(pos).any(|x| is_symbol(x))
    }

    fn get_number(&self, pos: &SerialPosition) -> u32 {
        self.field[pos.y][pos.x..pos.x + pos.length]
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }

    fn sum_serial(&self) -> u32 {
        self.find_serial()
            .filter(|x| self.is_serial(&x))
            .map(|x| self.get_number(&x))
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct SerialPosition {
    x: usize,
    y: usize,
    length: usize,
}

struct SerialIterator<'a> {
    schema: &'a Schema,
    pos: SerialPosition,
}

fn is_symbol(value: char) -> bool {
    !value.is_numeric() && value != '.'
}

impl Iterator for SerialIterator<'_> {
    type Item = SerialPosition;

    // Find start position and length of each serial number
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self
                .schema
                .get(self.pos.x as i32, self.pos.y as i32)
                .is_numeric()
            {
                // found start of a number
                loop {
                    self.pos.length += 1;
                    if !self
                        .schema
                        .get((self.pos.x + self.pos.length) as i32, self.pos.y as i32)
                        .is_numeric()
                    {
                        break;
                    }
                }

                let result = self.pos;
                self.pos.x += self.pos.length;
                self.pos.length = 0;

                return Some(result);
            } else {
                self.pos.x += 1;
                if self.pos.x >= self.schema.len_x() {
                    self.pos.x = 0;
                    self.pos.y += 1;
                }
                if self.pos.y >= self.schema.len_y() {
                    return None;
                }
            }
        }
    }
}

struct AdjacentIterator<'a> {
    schema: &'a Schema,
    pos: &'a SerialPosition,
    x: i32,
    y: i32,
}

impl Iterator for AdjacentIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == -2 {
            self.x = self.pos.x as i32 - 1;
            self.y = self.pos.y as i32 - 1;

            return Some(self.schema.get(self.x, self.y));
        }
        self.x += 1;
        if self.x > (self.pos.x + self.pos.length) as i32 {
            self.x = self.pos.x as i32 - 1;
            self.y += 1;
        }
        if self.y == self.pos.y as i32
            && self.x >= self.pos.x as i32
            && self.x <= (self.pos.x + self.pos.length - 1) as i32
        {
            self.x = (self.pos.x + self.pos.length) as i32;
        }
        if self.y > (self.pos.y + 1) as i32 {
            return None;
        }

        Some(self.schema.get(self.x, self.y))
    }
}

fn main() {
    let input = fs::read_to_string("3.input").expect("Should have been able to read the file");
    let s = Schema::new(&input);
    println!("Sum of serials: {}", s.sum_serial());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_schema_new() {
        assert_eq!(
            Schema::new("467..114..\n...*......\n"),
            Schema {
                field: vec![
                    vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
                    vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
                ]
            }
        )
    }

    #[test]
    fn test_get() {
        let s = Schema::new("467..114.9\n...*......\n");

        assert_eq!(s.get(-1, -1), '.');
        assert_eq!(s.get(0, 0), '4');
        assert_eq!(s.get(1, 0), '6');
        assert_eq!(s.get(9, 0), '9');
        assert_eq!(s.get(10, 0), '.');
        assert_eq!(s.get(0, 1), '.');
        assert_eq!(s.get(3, 1), '*');
        assert_eq!(s.get(0, 2), '.');
    }

    #[test]
    fn test_len() {
        let s = Schema::new("467..114.9\n...*......\n");

        assert_eq!(s.len_x(), 10);
        assert_eq!(s.len_y(), 2);
    }

    #[test]
    fn test_find_serial() {
        let s = Schema::new("467..114.9\n...*......\n..35..633.\n");

        let mut iter = s.find_serial();

        assert_eq!(
            iter.next(),
            Some(SerialPosition {
                x: 0,
                y: 0,
                length: 3
            })
        );
        assert_eq!(
            iter.next(),
            Some(SerialPosition {
                x: 5,
                y: 0,
                length: 3
            })
        );
        assert_eq!(
            iter.next(),
            Some(SerialPosition {
                x: 9,
                y: 0,
                length: 1
            })
        );
        assert_eq!(
            iter.next(),
            Some(SerialPosition {
                x: 2,
                y: 2,
                length: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(SerialPosition {
                x: 6,
                y: 2,
                length: 3
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_find_adjacent() {
        let s = Schema::new("467..114.9\n...*......\n");
        let Some(serial) = s.find_serial().next() else {
            panic!("broken text")
        };
        let mut adjacent_iter = s.find_adjacent(&serial);

        for _ in 1..12 {
            assert_eq!(adjacent_iter.next(), Some('.'));
        }
        assert_eq!(adjacent_iter.next(), Some('*'));
        assert_eq!(adjacent_iter.next(), None);
    }

    #[test]
    fn test_is_serial() {
        let input =
            fs::read_to_string("3sample.input").expect("Should have been able to read the file");
        let s = Schema::new(&input);
        let Some(serial) = s.find_serial().next() else {
            panic!("broken text")
        };

        assert_eq!(s.is_serial(&serial), true);
    }

    #[test]
    fn test_get_number() {
        let s = Schema::new("467..114.9\n...*......\n");
        let Some(serial) = s.find_serial().next() else {
            panic!("broken text")
        };

        assert_eq!(s.get_number(&serial), 467);
    }

    #[test]
    fn test_sum_serial() {
        let input =
            fs::read_to_string("3sample.input").expect("Should have been able to read the file");
        let s = Schema::new(&input);

        assert_eq!(s.sum_serial(), 4361);
    }
}
