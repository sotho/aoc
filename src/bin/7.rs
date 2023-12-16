use enum_map::{enum_map, Enum};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Enum, Clone, Copy)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Enum, Clone, Copy)]
enum Card2 {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from_byte(s: u8) -> Self {
        match s {
            b'2' => Card::C2,
            b'3' => Card::C3,
            b'4' => Card::C4,
            b'5' => Card::C5,
            b'6' => Card::C6,
            b'7' => Card::C7,
            b'8' => Card::C8,
            b'9' => Card::C9,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => panic!(),
        }
    }
}

impl Card2 {
    fn from_byte(s: u8) -> Self {
        match s {
            b'J' => Card2::J,
            b'2' => Card2::C2,
            b'3' => Card2::C3,
            b'4' => Card2::C4,
            b'5' => Card2::C5,
            b'6' => Card2::C6,
            b'7' => Card2::C7,
            b'8' => Card2::C8,
            b'9' => Card2::C9,
            b'T' => Card2::T,
            b'Q' => Card2::Q,
            b'K' => Card2::K,
            b'A' => Card2::A,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandTypeEnum {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandTypeEnum {
    fn get_type(cards: &[Card; 5]) -> Self {
        let mut card_map = enum_map! {
            Card::C2 => 0,
            Card::C3 => 0,
            Card::C4 => 0,
            Card::C5 => 0,
            Card::C6 => 0,
            Card::C7 => 0,
            Card::C8 => 0,
            Card::C9 => 0,
            Card::T => 0,
            Card::J => 0,
            Card::Q => 0,
            Card::K => 0,
            Card::A => 0,
        };
        cards.iter().for_each(|card| card_map[*card] += 1);
        if card_map.iter().any(|(_card, &value)| value == 5) {
            return HandTypeEnum::FiveOfAKind;
        } else if card_map.iter().any(|(_card, &value)| value == 4) {
            return HandTypeEnum::FourOfAKind;
        } else if card_map.iter().any(|(_card, &value)| value == 3)
            && card_map.iter().any(|(_card, &value)| value == 2)
        {
            return HandTypeEnum::FullHouse;
        } else if card_map.iter().any(|(_card, &value)| value == 3) {
            return HandTypeEnum::ThreeOfAKind;
        } else if card_map.iter().filter(|(_card, &value)| value == 2).count() == 2 {
            return HandTypeEnum::TwoPair;
        } else if card_map.iter().filter(|(_card, &value)| value == 1).count() == 3 {
            return HandTypeEnum::OnePair;
        } else if card_map.iter().filter(|(_card, &value)| value == 1).count() == 5 {
            return HandTypeEnum::HighCard;
        } else {
            panic!();
        }
    }

    fn get_type2(cards: &[Card2; 5]) -> Self {
        let mut card_map = enum_map! {
            Card2::C2 => 0,
            Card2::C3 => 0,
            Card2::C4 => 0,
            Card2::C5 => 0,
            Card2::C6 => 0,
            Card2::C7 => 0,
            Card2::C8 => 0,
            Card2::C9 => 0,
            Card2::T => 0,
            Card2::J => 0,
            Card2::Q => 0,
            Card2::K => 0,
            Card2::A => 0,
        };

        cards.iter().for_each(|card| card_map[*card] += 1);

        let jokers = card_map[Card2::J];

        if card_map.iter().any(|(card, &value)| match card {
            Card2::J => value == 5,
            _ => value + jokers == 5,
        }) {
            return HandTypeEnum::FiveOfAKind;
        } else if card_map.iter().any(|(card, &value)| match card {
            Card2::J => value == 4,
            _ => value + jokers == 4,
        }) {
            return HandTypeEnum::FourOfAKind;
        } else if (jokers == 0
            && card_map.iter().any(|(_card, &value)| value == 3)
            && card_map.iter().any(|(_card, &value)| value == 2))
            || (jokers == 1 && card_map.iter().filter(|(_card, &value)| value == 2).count() == 2)
        {
            // if we have 2 jokers, then we would need 2 of the same to be able to get a FullHouse, but this is FourOfAKind
            return HandTypeEnum::FullHouse;
        } else if card_map.iter().any(|(card, &value)| match card {
            Card2::J => value == 3,
            _ => value + jokers == 3,
        }) {
            return HandTypeEnum::ThreeOfAKind;
        } else if jokers == 0 && card_map.iter().filter(|(_card, &value)| value == 2).count() == 2 {
            // with 2 jokers it would be at least ThreeOfAKind
            // with 1 joker we would need a Pair and a single card + joker to get TwoPair, but this is ThreeOfAKind
            // => no jokers
            return HandTypeEnum::TwoPair;
        } else if (jokers == 0 && card_map.iter().filter(|(_card, &value)| value == 1).count() == 3)
            || jokers == 1
        {
            // a joker with any single card is always a OnePair
            return HandTypeEnum::OnePair;
        } else if card_map.iter().filter(|(_card, &value)| value == 1).count() == 5 {
            return HandTypeEnum::HighCard;
        } else {
            panic!();
        }
    }
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Eq)]
struct Hand2 {
    cards: [Card2; 5],
    bid: u64,
}

impl Hand {
    fn new(value: &str) -> Vec<Self> {
        value
            .split_terminator('\n')
            .map(|line| {
                let mut iter = line.split(' ');
                Hand {
                    cards: iter
                        .next()
                        .unwrap()
                        .bytes()
                        .map(|card| Card::from_byte(card))
                        .collect::<Vec<Card>>()
                        .try_into()
                        .unwrap(),
                    bid: iter.next().unwrap().parse::<u64>().unwrap(),
                }
            })
            .collect()
    }
}

impl Hand2 {
    fn new(value: &str) -> Vec<Self> {
        value
            .split_terminator('\n')
            .map(|line| {
                let mut iter = line.split(' ');
                Hand2 {
                    cards: iter
                        .next()
                        .unwrap()
                        .bytes()
                        .map(|card| Card2::from_byte(card))
                        .collect::<Vec<Card2>>()
                        .try_into()
                        .unwrap(),
                    bid: iter.next().unwrap().parse::<u64>().unwrap(),
                }
            })
            .collect()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_type = HandTypeEnum::get_type(&self.cards);
        let other_type = HandTypeEnum::get_type(&other.cards);

        if this_type == other_type {
            return self.cards.cmp(&other.cards);
        } else {
            return this_type.cmp(&other_type);
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_type = HandTypeEnum::get_type2(&self.cards);
        let other_type = HandTypeEnum::get_type2(&other.cards);

        if this_type == other_type {
            return self.cards.cmp(&other.cards);
        } else {
            return this_type.cmp(&other_type);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn win(mut hands: Vec<Hand>) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.bid)
        .sum()
}

fn win2(mut hands: Vec<Hand2>) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.bid)
        .sum()
}

fn main() {
    let input = fs::read_to_string("7.input").expect("Should have been able to read the file");

    let hand = Hand::new(&input);
    println!("win: {}", win(hand));

    let hand2 = Hand2::new(&input);
    println!("win2: {}", win2(hand2));

}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_hand() -> Vec<Hand> {
        let input =
            fs::read_to_string("7sample.input").expect("Should have been able to read the file");

        Hand::new(&input)
    }

    fn get_hand2() -> Vec<Hand2> {
        let input =
            fs::read_to_string("7sample.input").expect("Should have been able to read the file");

        Hand2::new(&input)
    }

    #[test]
    fn test_hand_new() {
        let hand = get_hand();

        assert_eq!(hand.len(), 5);
        assert_eq!(
            hand[0].cards,
            [Card::C3, Card::C2, Card::T, Card::C3, Card::K]
        );
        assert_eq!(hand[0].bid, 765);
    }

    #[test]
    fn test_hand2_new() {
        let hand = get_hand2();

        assert_eq!(hand.len(), 5);
        assert_eq!(
            hand[0].cards,
            [Card2::C3, Card2::C2, Card2::T, Card2::C3, Card2::K]
        );
        assert_eq!(hand[0].bid, 765);
    }

    #[test]
    fn test_card_type() {
        use Card::*;

        assert_eq!(
            HandTypeEnum::get_type(&[A, A, A, A, A]),
            HandTypeEnum::FiveOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type(&[A, A, C8, A, A]),
            HandTypeEnum::FourOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type(&[C2, C3, C3, C3, C2]),
            HandTypeEnum::FullHouse
        );
        assert_eq!(
            HandTypeEnum::get_type(&[T, T, T, C9, C8]),
            HandTypeEnum::ThreeOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type(&[C2, C3, C4, C3, C2]),
            HandTypeEnum::TwoPair
        );
        assert_eq!(
            HandTypeEnum::get_type(&[A, C2, C3, A, C4]),
            HandTypeEnum::OnePair
        );
        assert_eq!(
            HandTypeEnum::get_type(&[C2, C3, C4, C5, C6]),
            HandTypeEnum::HighCard
        );
    }

    #[test]
    fn test_card2_type() {
        use Card2::*;

        assert_eq!(
            HandTypeEnum::get_type2(&[C3, C2, T, C3, K]),
            HandTypeEnum::OnePair
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[K, K, C6, C7, C7]),
            HandTypeEnum::TwoPair
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[T, C5, C5, J, C5]),
            HandTypeEnum::FourOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[K, T, J, J, T]),
            HandTypeEnum::FourOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[Q, Q, Q, J, A]),
            HandTypeEnum::FourOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[C2, A, C4, J, J]),
            HandTypeEnum::ThreeOfAKind
        );
        assert_eq!(
            HandTypeEnum::get_type2(&[C2, A, C4, J, T]),
            HandTypeEnum::OnePair
        );
    }

    #[test]
    fn test_ord() {
        use Card::*;

        assert!(
            Hand {
                cards: [A, A, A, A, A],
                bid: 0
            } > Hand {
                cards: [A, A, C8, A, A],
                bid: 0
            }
        );

        assert!(
            Hand {
                cards: [C3, C3, C3, C3, C2],
                bid: 0
            } > Hand {
                cards: [C2, A, A, A, A],
                bid: 0
            }
        );

        assert!(
            Hand {
                cards: [C7, C7, C8, C8, C8],
                bid: 0
            } > Hand {
                cards: [C7, C7, C7, C8, C8],
                bid: 0
            }
        );
    }

    #[test]
    fn test_rank() {
        let mut hand = get_hand();
        hand.sort();

        assert_eq!(hand[0].bid, 765);
        assert_eq!(hand[1].bid, 220);
        assert_eq!(hand[2].bid, 28);
        assert_eq!(hand[3].bid, 684);
        assert_eq!(hand[4].bid, 483);
    }

    #[test]
    fn test_win() {
        let hand = get_hand();

        assert_eq!(win(hand), 6440);
    }

    #[test]
    fn test_win2() {
        let hand = get_hand2();

        assert_eq!(win2(hand), 5905);
    }
}
