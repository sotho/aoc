use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Card {
    cardnum: u8,
    win: Vec<u8>,
    got: Vec<u8>,
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
        }
    }

    fn points(&self) -> u32 {
        let count: u32 = self
            .got
            .iter()
            .filter(|x| self.win.contains(x))
            .count()
            .try_into()
            .unwrap();

        match count {
            0 => 0,
            _ => 2_u32.pow(count - 1),
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

    fn points(&self) -> u32 {
        self.cards.iter().map(|x| x.points()).sum()
    }
}

fn main() {
    let input = fs::read_to_string("4.input").expect("Should have been able to read the file");
    let cards = Cards::new(&input);

    println!("total points: {}", cards.points());
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
}
