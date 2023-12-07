advent_of_code::solution!(7);

use std::cmp::Ordering::*;

#[derive(PartialEq, PartialOrd)]
struct Cards(u32, u32, u32, u32, u32);

#[derive(PartialEq, PartialOrd)]
enum Hands {
    // Ordering from low to high due to rust discriminants
    HighCard(Cards, u32),
    Pair(Cards, u32),
    TwoPairs(Cards, u32),
    Three(Cards, u32),
    FullHouse(Cards, u32),
    Four(Cards, u32),
    Five(Cards, u32),
}

impl Hands {
    fn from_card_counts(counts: &[u32], cards: Cards, bid: u32) -> Hands {
        if counts.iter().max().unwrap() == &5 {
            Hands::Five(cards, bid)
        } else if counts.iter().max().unwrap() == &4 {
            Hands::Four(cards, bid)
        } else if counts.iter().filter(|&&x| x > 0).count() == 2 {
            Hands::FullHouse(cards, bid)
        } else if counts.iter().max().unwrap() == &3 {
            Hands::Three(cards, bid)
        } else if counts.iter().filter(|&&x| x > 0).count() == 3 {
            // 11223 or 12233 or 111233
            Hands::TwoPairs(cards, bid)
        } else if counts.iter().filter(|&&x| x > 0).count() == 4 {
            Hands::Pair(cards, bid)
        } else {
            Hands::HighCard(cards, bid)
        }
    }

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

        let mut card_counts: [u32; 14] = [0; 14];
        cards.iter().for_each(|&c| card_counts[c as usize - 1] += 1);

        Hands::from_card_counts(
            &card_counts,
            Cards(cards[0], cards[1], cards[2], cards[3], cards[4]),
            bid,
        )
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

        let mut card_counts: [u32; 15] = [0; 15];
        cards.iter().for_each(|&c| card_counts[c as usize] += 1);
        let jokers = card_counts[0];
        let non_jokers = &mut card_counts[1..];
        // Heuristic: Best hand is obtained by adding joker to higher card count
        *(non_jokers.iter_mut().max().unwrap()) += jokers;

        Hands::from_card_counts(
            non_jokers,
            Cards(cards[0], cards[1], cards[2], cards[3], cards[4]),
            bid,
        )
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
