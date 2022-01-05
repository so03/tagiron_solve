use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Cards {
    v: Vec<Card>,
}

impl Cards {
    pub fn new(v: Vec<Card>) -> Self {
        Cards { v }
    }

    pub fn all() -> Self {
        Cards {
            v: ALL_CARDS.to_vec(),
        }
    }

    pub fn all_combs() -> Vec<Cards> {
        ALL_CARDS
            .into_iter()
            .combinations(4)
            .map(Cards::new)
            .collect()
    }

    pub fn combs(&self) -> Vec<Cards> {
        self.v
            .clone()
            .into_iter()
            .combinations(4)
            .map(Cards::new)
            .collect()
    }

    fn nums(&self) -> Vec<usize> {
        self.v.iter().map(|c| c.number).collect()
    }

    pub fn sum(&self) -> usize {
        self.v.iter().fold(0, |a, c| a + c.number)
    }

    pub fn odds(&self) -> impl Iterator<Item = &Card> {
        self.v.iter().filter(|c| c.number % 2 == 1)
    }

    pub fn big_three_sum(&self) -> usize {
        let mut nums = self.nums();
        nums.sort_unstable();
        nums[1..].iter().sum()
    }

    pub fn small_three_sum(&self) -> usize {
        let mut nums = self.nums();
        nums.sort_unstable();
        nums[0..3].iter().sum()
    }

    pub fn evens(&self) -> impl Iterator<Item = &Card> {
        self.v.iter().filter(|v| v.number % 2 == 0)
    }

    /// Returns index-list of given number.
    pub fn positions_of(&self, num: usize) -> Vec<usize> {
        self.v
            .iter()
            .enumerate()
            .map(|(i, c)| if c.number == num { Some(i) } else { None })
            .flatten()
            .collect()
    }

    pub fn reds(&self) -> impl Iterator<Item = &Card> {
        self.v.iter().filter(|c| c.color == Color::Red)
    }

    pub fn blues(&self) -> impl Iterator<Item = &Card> {
        self.v.iter().filter(|c| c.color == Color::Blue)
    }

    pub fn red_sums(&self) -> usize {
        self.reds().fold(0, |a, c| a + c.number)
    }

    pub fn blue_sums(&self) -> usize {
        self.blues().fold(0, |a, c| a + c.number)
    }

    pub fn red_counts(&self) -> usize {
        self.reds().count()
    }

    pub fn blue_counts(&self) -> usize {
        self.blues().count()
    }

    pub fn is_same_color(&self, a: usize, b: usize) -> bool {
        self.v[a].color == self.v[b].color
    }

    pub fn iter(&self) -> std::slice::Iter<Card> {
        self.v.iter()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn difference(&self, other: &Cards) -> Cards {
        let mut other = other.clone();
        let filtered = self
            .v
            .iter()
            .filter(|&c| {
                if let Some(i) = other.iter().position(|u| u == c) {
                    other.v.remove(i);
                    false
                } else {
                    true
                }
            })
            .copied()
            .collect_vec();
        Cards { v: filtered }
    }
}

impl Eq for Cards {}

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        if self.v.len() != other.v.len() {
            panic!("Can't compare cards. They have different lengths.")
        }
        self.v
            .iter()
            .sorted()
            .zip(other.v.iter().sorted())
            .all(|(one, other)| one == other)
    }
}

// Hash implementation

// impl Hash for Cards {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         for c in self.v.iter().sorted() {
//             c.hash(state);
//         }
//     }
// }

// #[test]
// fn test_cards_hash() {
//     let cards1 = Cards::new(vec![
//         Card {
//             number: 1,
//             color: Color::Blue,
//         },
//         Card {
//             number: 2,
//             color: Color::Red,
//         },
//         Card {
//             number: 4,
//             color: Color::Red,
//         },
//         Card {
//             number: 5,
//             color: Color::Yellow,
//         },
//     ]);
//     let cards2 = Cards::new(vec![
//         Card {
//             number: 5,
//             color: Color::Yellow,
//         },
//         Card {
//             number: 1,
//             color: Color::Blue,
//         },
//         Card {
//             number: 4,
//             color: Color::Red,
//         },
//         Card {
//             number: 2,
//             color: Color::Red,
//         },
//     ]);

//     assert_eq!(calculate_hash(&cards1), calculate_hash(&cards2));

//     fn calculate_hash<T: Hash>(t: &T) -> u64 {
//         let mut s = DefaultHasher::new();
//         t.hash(&mut s);
//         s.finish()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub(crate) number: usize,
    pub(crate) color: Color,
}

impl Card {
    pub fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    pub fn is_blue(&self) -> bool {
        self.color == Color::Blue
    }

    pub fn is_yellow(&self) -> bool {
        self.color == Color::Yellow
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red,
    Blue,
    Yellow,
}

pub const ALL_CARDS: [Card; 20] = [
    Card {
        number: 0,
        color: Color::Red,
    },
    Card {
        number: 1,
        color: Color::Red,
    },
    Card {
        number: 2,
        color: Color::Red,
    },
    Card {
        number: 3,
        color: Color::Red,
    },
    Card {
        number: 4,
        color: Color::Red,
    },
    Card {
        number: 5,
        color: Color::Yellow,
    },
    Card {
        number: 6,
        color: Color::Red,
    },
    Card {
        number: 7,
        color: Color::Red,
    },
    Card {
        number: 8,
        color: Color::Red,
    },
    Card {
        number: 9,
        color: Color::Red,
    },
    Card {
        number: 0,
        color: Color::Blue,
    },
    Card {
        number: 1,
        color: Color::Blue,
    },
    Card {
        number: 2,
        color: Color::Blue,
    },
    Card {
        number: 3,
        color: Color::Blue,
    },
    Card {
        number: 4,
        color: Color::Blue,
    },
    Card {
        number: 5,
        color: Color::Yellow,
    },
    Card {
        number: 6,
        color: Color::Blue,
    },
    Card {
        number: 7,
        color: Color::Blue,
    },
    Card {
        number: 8,
        color: Color::Blue,
    },
    Card {
        number: 9,
        color: Color::Blue,
    },
];
