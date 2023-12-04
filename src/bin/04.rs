advent_of_code::solution!(4);

struct Card {
    winning: Vec<u32>,
    own: Vec<u32>,
    copies: u32,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let (winning, own) = line.split_once(":").unwrap().1.split_once("|").unwrap();
        Card {
            winning: winning
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            own: own
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            copies: 1,
        }
    }

    fn add_copies(&mut self, copies: u32) {
        self.copies += copies;
    }

    fn winning_nums(&self) -> usize {
        self.own.iter().filter(|x| self.winning.contains(x)).count()
    }

    fn score(&self) -> u32 {
        let n = self.winning_nums();
        if n == 0 {
            return 0;
        }
        2_u32.pow(n.wrapping_sub(1).try_into().unwrap())
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(Card::from_line).map(|c| c.score()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = input.lines().map(Card::from_line).collect();
    for i in 0..cards.len() {
        let copies = cards[i].copies;
        for j in 0..cards[i].winning_nums() {
            cards[i + j + 1].add_copies(copies);
        }
    }
    Some(cards.iter().map(|x| x.copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
