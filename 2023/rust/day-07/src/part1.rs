use crate::custom_error::AocError;
use itertools::Itertools;
use std::cmp::Ordering;
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Card {
    // Value ascending order
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Ranking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        let cards = cards
            .chars()
            .map(|s| s.to_string().parse::<Card>().expect("Failed to parse card"))
            .collect::<Vec<Card>>();
        Self { cards, bid }
    }

    fn get_ranking(&self) -> Ranking {
        let mut collection = self.cards.iter().sorted().dedup_with_count().sorted().rev();
        let (most_count, _) = collection.next().expect("No cards found");
        let (second_count, _) = collection
            .next()
            .or(Some((0, &Card::Two)))
            .expect("No cards found");

        match (most_count, second_count) {
            (5, _) => Ranking::FiveOfAKind,
            (4, _) => Ranking::FourOfAKind,
            (3, 2) => Ranking::FullHouse,
            (3, _) => Ranking::ThreeOfAKind,
            (2, 2) => Ranking::TwoPair,
            (2, _) => Ranking::OnePair,
            (1, _) => Ranking::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_ranking().cmp(&other.get_ranking()) {
            Ordering::Equal => {
                let mut self_cards = self.cards.iter();
                let mut other_cards = other.cards.iter();
                loop {
                    match (self_cards.next(), other_cards.next()) {
                        (Some(self_card), Some(other_card)) => match (self_card).cmp(other_card) {
                            Ordering::Equal => continue,
                            ordering => {
                                return ordering;
                            }
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let sum: u32 = _input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cards = parts.next().unwrap();
            let bid = parts
                .next()
                .unwrap()
                .parse::<u32>()
                .expect("Failed to parse bid");

            Hand::new(cards, bid)
        })
        .sorted()
        .enumerate()
        .map(|(i, hand)| ((i as u32 + 1) * hand.bid))
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }

    #[test]
    fn test_hand_get_ranking() -> miette::Result<()> {
        let hand = Hand::new("32T3K", 765);
        assert_eq!(Ranking::OnePair, hand.get_ranking());
        let hand = Hand::new("AAAAA", 765);
        assert_eq!(Ranking::FiveOfAKind, hand.get_ranking());
        let hand = Hand::new("22333", 765);
        assert_eq!(Ranking::FullHouse, hand.get_ranking());
        let hand = Hand::new("23456", 765);
        assert_eq!(Ranking::HighCard, hand.get_ranking());
        let hand = Hand::new("43455", 765);
        assert_eq!(Ranking::TwoPair, hand.get_ranking());
        Ok(())
    }

    #[test]
    fn test_hand_compare() -> miette::Result<()> {
        let hand1 = Hand::new("32T3K", 765);
        let hand2 = Hand::new("T55J5", 684);
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        let hand1 = Hand::new("KK677", 28);
        let hand2 = Hand::new("KTJJT", 220);
        assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
        let hand1 = Hand::new("22222", 28);
        let hand2 = Hand::new("66666", 220);
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        let hand1 = Hand::new("KAAAA", 28);
        let hand2 = Hand::new("AAAAK", 220);
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        Ok(())
    }
}
