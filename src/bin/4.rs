use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Card {
    cardnum: u8,
    win: Vec<u8>,
    got: Vec<u8>,
    count: u32,
}

impl Card {
    fn new(value: &str) -> Self {
        let re = Regex::new(r"^Card +([0-9]+): +(.*) +\| +(.*) *$").unwrap();
        let caps = re.captures(value).unwrap();

        Card {
            cardnum: caps[1].parse::<u8>().unwrap(),
            win: caps[2]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
            got: caps[3]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
            count: 1,
        }
    }

    fn wins(&self) -> u32 {
        self.got
            .iter()
            .filter(|x| self.win.contains(x))
            .count()
            .try_into()
            .unwrap()
    }

    fn points(&self) -> u32 {
        let wins: u32 = self.wins();

        match wins {
            0 => 0,
            _ => 2_u32.pow(wins - 1),
        }
    }
}

struct Cards {
    cards: Vec<Card>,
}

impl Cards {
    fn new(value: &str) -> Cards {
        Cards {
            cards: value
                .split_terminator('\n')
                .map(|line| Card::new(line))
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn points(&self) -> u32 {
        self.cards.iter().map(|x| x.points()).sum()
    }

    fn calc_copies(mut self) -> Self {
        for i in 0..self.len() {
            for index in self.cards[i].cardnum as usize
                ..self.cards[i].cardnum as usize + self.cards[i].wins() as usize
            {
                if index >= self.len() {
                    break;
                }
                self.cards[index].count += self.cards[i].count;
            }
        }
        self
    }

    fn count(&self) -> u32 {
        self.cards.iter().map(|card| card.count).sum()
    }
}

fn main() {
    let input = fs::read_to_string("4.input").expect("Should have been able to read the file");
    let cards = Cards::new(&input).calc_copies();

    println!("total points: {}", cards.points());
    println!("total cards: {}", cards.count());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_card_new() {
        assert_eq!(
            Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                cardnum: 1,
                win: vec![41, 48, 83, 86, 17],
                got: vec![83, 86, 6, 31, 17, 9, 48, 53],
                count: 1
            }
        );
    }
    #[test]
    fn test_cards_new() {
        let input =
            fs::read_to_string("4sample.input").expect("Should have been able to read the file");
        let cards = Cards::new(&input);
        assert_eq!(
            cards.cards[0],
            Card {
                cardnum: 1,
                win: vec![41, 48, 83, 86, 17],
                got: vec![83, 86, 6, 31, 17, 9, 48, 53],
                count: 1
            }
        );

        assert_eq!(cards.cards.len(), 6);
    }

    #[test]
    fn test_card_points() {
        let card = Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(card.points(), 8);
    }

    #[test]
    fn test_cards_points() {
        let input =
            fs::read_to_string("4sample.input").expect("Should have been able to read the file");
        let cards = Cards::new(&input);

        assert_eq!(cards.points(), 13);
    }

    #[test]
    fn test_calc_copies() {
        let input =
            fs::read_to_string("4sample.input").expect("Should have been able to read the file");
        let cards = Cards::new(&input).calc_copies();

        assert_eq!(cards.cards[0].count, 1);
        assert_eq!(cards.cards[1].count, 2);
        assert_eq!(cards.cards[2].count, 4);
        assert_eq!(cards.cards[3].count, 8);
        assert_eq!(cards.cards[4].count, 14);
        assert_eq!(cards.cards[5].count, 1);
    }

    #[test]
    fn test_cards_count() {
        let input =
            fs::read_to_string("4sample.input").expect("Should have been able to read the file");
        let cards = Cards::new(&input);
        let cards = cards.calc_copies();

        assert_eq!(cards.count(), 30);
    }
}
