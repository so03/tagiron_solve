pub mod executor;
pub mod models;
pub mod functions;

use executor::*;
use itertools::Itertools;

use crate::models::ALL_CARDS;

fn main() {
    let all_combs = ALL_CARDS.into_iter().combinations(4).collect();

    let executor = Executor::new();

    let queries = vec![("sum red", vec![6]), ("how many red", vec![4])];

    let result = executor.run(all_combs, queries);

    println!("{:?}", result);
}

#[test]
fn count_of_all_combs() {
    let all_combs = ALL_CARDS.iter().combinations(4);
    assert_eq!(4845, all_combs.count());
}
