advent_of_code::solution!(7);

use std::cmp::Ordering::*;
struct Cards(u32, u32, u32, u32, u32);

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
            && self.4 == other.4
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cards_other = vec![other.0, other.1, other.2, other.3, other.4];
        let cards = vec![self.0, self.1, self.2, self.3, self.4];

        cards
            .iter()
            .zip(cards_other)
            .find_map(|(&a, b)| match a.cmp(&b) {
                std::cmp::Ordering::Equal => None,
                x => Some(x),
            })
    }
}
enum Hands {
    Five(Cards, u32),
    Four(Cards, u32),
    FullHouse(Cards, u32),
    Three(Cards, u32),
    TwoPairs(Cards, u32),
    Pair(Cards, u32),
    HighCard(Cards, u32),
}

impl Hands {
    fn from_line(line: &str) -> Hands {
        let (line, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let cards: Vec<u32> = line
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap_or(match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
            })
            .collect();

        let mut card_counts: Vec<u32> = vec![0; 14];
        cards.iter().for_each(|&c| card_counts[c as usize - 1] += 1);

        // Check for 5 of a kind
        if card_counts.iter().max().unwrap() == &5 {
            Hands::Five(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if card_counts.iter().max().unwrap() == &4 {
            Hands::Four(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if card_counts.iter().filter(|&&x| x > 0).count() == 2 {
            Hands::FullHouse(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if card_counts.iter().max().unwrap() == &3 {
            Hands::Three(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if card_counts.iter().filter(|&&x| x > 0).count() == 3 {
            // 11223 or 12233 or 111233
            Hands::TwoPairs(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if card_counts.iter().filter(|&&x| x > 0).count() == 4 {
            Hands::Pair(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else {
            Hands::HighCard(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        }
    }
    fn from_line_joker(line: &str) -> Hands {
        let (line, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let cards: Vec<u32> = line
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap_or(match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 0,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
            })
            .collect();

        let mut card_counts: Vec<u32> = vec![0; 15];
        cards.iter().for_each(|&c| card_counts[c as usize] += 1);
        let non_jokers = &card_counts[1..];
        let jokers = card_counts[0];

        if non_jokers.iter().max().unwrap() + jokers == 5 {
            Hands::Five(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if non_jokers.iter().max().unwrap() + jokers == 4 {
            Hands::Four(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if non_jokers.iter().filter(|&&x| x > 0).count() == 2 {
            // aaabb, aabbj, abbjj, abcjj
            Hands::FullHouse(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if non_jokers.iter().max().unwrap() + jokers == 3 {
            Hands::Three(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if non_jokers.iter().filter(|&&x| x > 0).count() == 3 {
            // aaddc
            Hands::TwoPairs(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else if non_jokers.iter().filter(|&&x| x > 0).count() == 4 {
            Hands::Pair(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        } else {
            Hands::HighCard(Cards(cards[0], cards[1], cards[2], cards[3], cards[4]), bid)
        }
    }

    fn bid(&self) -> u32 {
        match self {
            Hands::Five(_, bid) => *bid,
            Hands::Four(_, bid) => *bid,
            Hands::FullHouse(_, bid) => *bid,
            Hands::Three(_, bid) => *bid,
            Hands::TwoPairs(_, bid) => *bid,
            Hands::Pair(_, bid) => *bid,
            Hands::HighCard(_, bid) => *bid,
        }
    }

    fn cards(&self) -> &Cards {
        match self {
            Hands::Five(cards, _) => cards,
            Hands::Four(cards, _) => cards,
            Hands::FullHouse(cards, _) => cards,
            Hands::Three(cards, _) => cards,
            Hands::TwoPairs(cards, _) => cards,
            Hands::Pair(cards, _) => cards,
            Hands::HighCard(cards, _) => cards,
        }
    }

    fn hand_rank(&self) -> u32 {
        match self {
            Hands::Five(_, _) => 6,
            Hands::Four(_, _) => 5,
            Hands::FullHouse(_, _) => 4,
            Hands::Three(_, _) => 3,
            Hands::TwoPairs(_, _) => 2,
            Hands::Pair(_, _) => 1,
            Hands::HighCard(_, _) => 0,
        }
    }
}

impl PartialEq for Hands {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_rank() != other.hand_rank() {
            false
        } else {
            self.cards() == other.cards()
        }
    }
}

impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_rank().cmp(&other.hand_rank()) {
            Less => Some(Less),
            Greater => Some(Greater),
            Equal => self.cards().partial_cmp(other.cards()),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hands> = input.lines().map(Hands::from_line).collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let tot_bids = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid())
        .sum::<u32>()
        .into();
    Some(tot_bids)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hands> = input.lines().map(Hands::from_line_joker).collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let tot_bids = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid())
        .sum::<u32>()
        .into();
    Some(tot_bids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
