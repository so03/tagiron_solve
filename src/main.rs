pub mod executor;
pub mod functions;
pub mod models;
pub mod resolver;

use executor::*;
use itertools::Itertools;
use models::Card;

use crate::models::ALL_CARDS;

use std::collections::HashSet;

fn main() {}

#[test]
fn count_of_all_combs() {
    let all_combs = ALL_CARDS.iter().combinations(4);
    assert_eq!(4845, all_combs.count());
}
