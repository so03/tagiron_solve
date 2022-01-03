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

    pub fn iter(&self) -> std::slice::Iter<Card> {
        self.v.iter()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn difference(&self, other: &Cards) -> Cards {
        let self_hs: HashSet<Card> = self.v.clone().into_iter().collect();
        let other_hs: HashSet<Card> = other.v.clone().into_iter().collect();
        let diff: Vec<Card> = self_hs.difference(&other_hs).copied().collect();
        Cards { v: diff }
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
