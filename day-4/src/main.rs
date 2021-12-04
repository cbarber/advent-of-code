use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input");

const PATTERNS: [u32; 10] = [
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001
    // lame bingo game doesn't count diag
    // 0b10000_01000_00100_00010_00001,
    // 0b00001_00010_00100_01000_10000,
];

#[derive(Debug, PartialEq)]
struct BingoCard {
    board: HashMap<u8, u8>,
    marked: u32,
}

impl BingoCard {
    fn new(input: &str) -> Self {
        let board = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .enumerate()
            .map(|a| (a.1, a.0 as u8))
            .collect::<HashMap<_, _>>();
        BingoCard {
            board,
            marked: 0u32,
        }
    }

    fn play(&mut self, draw: u8) {
        if let Some(index) = self.board.get(&draw) {
            self.marked |= 1 << index;
        }
    }

    fn is_winner(&self) -> bool {
        PATTERNS.iter().any(|p| p & self.marked == *p)
    }

    fn score(&self, last_num: u8) -> u32 {
        let unmarked_sum: u32 = self
            .board
            .iter()
            .filter(|(_, index)| self.marked & (1u32 << *index) == 0)
            .map(|(num, _)| *num as u32)
            .sum();
        unmarked_sum * last_num as u32
    }
}

#[derive(Debug)]
struct BingoGame {
    draw: VecDeque<u8>,
    cards: Vec<BingoCard>,
    last_draw: Option<u8>,
}

impl BingoGame {
    fn new(input: &str) -> Self {
        let mut iter = input.split("\n\n");
        let draw = iter
            .next()
            .unwrap_or("")
            .split(",")
            .filter_map(|s| s.parse().ok())
            .collect();
        let cards = iter.map(BingoCard::new).collect();

        Self {
            draw,
            cards,
            last_draw: None,
        }
    }

    fn play(&mut self) -> Option<u8> {
        self.last_draw = self.draw.pop_front();
        if let Some(draw) = self.last_draw {
            self.cards.iter_mut().for_each(|c| c.play(draw));
        }
        self.last_draw
    }

    fn find_winners(&self) -> Vec<&BingoCard> {
        self.cards.iter().filter(|c| c.is_winner()).collect()
    }
}

fn main() {
    let mut game = BingoGame::new(INPUT);
    while game.find_winners().is_empty() {
        game.play()
    }
    println!(
        "{}",
        game.find_winners()[0].score(game.last_draw.expect("drawn number"))
    );
        }
    };

    println!("{}", winners[0].score(last_drawn));
}

#[cfg(test)]
const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

#[test]
fn test_bingo_cards() {
    let mut game = BingoGame::new(TEST_INPUT);

    let no_winners: Vec<&BingoCard> = vec![];
    (0..11).into_iter().for_each(|_| {
        game.play();
        assert_eq!(no_winners, game.find_winners());
    });

    game.play();
    let winners = game.find_winners();
    assert_eq!(1, winners.len());
    assert_eq!(4512, winners[0].score(game.last_draw.unwrap()));
}
