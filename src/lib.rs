// To create and show Document
// cargo doc --no-deps --open
// With private items.
// cargo doc --no-deps --document-private-items --open

//! # Tagiron Solver
//!
//! ## Usage
//! ```
//! let q: Vec<Vec<tagiron_solve::Q>> = vec![
//!     vec![("what_is_your_answer", vec![1, 2, 3, 4, 1, 1, 1, 1])], // mine
//!     vec![("what_is_your_answer", vec![1, 2, 3, 4, 2, 2, 2, 2])],
//!     vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 1, 1, 1])],
//!     vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 2, 2, 2])],
//! ];
//!
//! tagiron_solve::solve(q);
//! ```

mod cards;
mod functions;

use cards::Cards;

use crate::functions::*;

pub type Q = (&'static str, Vec<usize>);

/// Write the answer to stdout if it finds out.
pub fn solve(q: Vec<Vec<Q>>) -> Result<Cards, &'static str> {
    // expected answer is: [0, 0, 9, 9, 1, 2, 1, 2]
    let cs = Cards::all();

    // all combinations
    let css = cs.combs();

    // expectations of answers
    let mut ap = vec![];

    let mut css1 = css;
    css1 = narrow_down(css1, q[0].clone());
    println!("css1: {:?}", css1);
    for cp1 in css1.iter() {
        let cs = cs.difference(cp1);
        let mut css2 = cs.combs();
        css2 = narrow_down(css2, q[1].clone());
        println!("css2: {:?}", css2);

        for cp2 in css2.iter() {
            let cs = cs.difference(cp2);
            let mut css3 = cs.combs();
            css3 = narrow_down(css3, q[2].clone());
            println!("css3: {:?}", css3);

            for cp3 in css3.iter() {
                let cs = cs.difference(cp3);
                println!("cs: {:?}", cs);
                let mut css4 = cs.combs();
                css4 = narrow_down(css4, q[3].clone());
                println!("css4: {:?}", css4);

                for cp4 in css4.iter() {
                    let ans = cs.difference(cp4);
                    println!("{:?}", ans);
                    if ans.len() == 4 {
                        ap.push(ans);
                    }
                }
            }
        }
    }

    ap.sort();
    ap.dedup();

    if ap.len() == 1 {
        Ok(ap[0].clone())
    } else {
        Err("Answer is not decided.")
    }
}

fn narrow_down(mut css: Vec<Cards>, qs: Vec<Q>) -> Vec<Cards> {
    for q in qs {
        css = query(q, css)
    }
    css
}

fn query(q: Q, css: Vec<Cards>) -> Vec<Cards> {
    css.into_iter()
        .filter(|v| match &q {
            ("what_is_your_answer", args) => what_is_your_answer(v, args.clone()),
            ("sum_red", args) => sum_red(v, args.clone()),
            ("sum_blue", args) => sum_blue(v, args.clone()),
            ("how_many_red", args) => how_many_red(v, args.clone()),
            ("where_is_zero", args) => where_is_zero(v, args.clone()),
            ("where_is_one", args) => where_is_one(v, args.clone()),
            ("where_is_two", args) => where_is_two(v, args.clone()),
            ("where_is_three", args) => where_is_three(v, args.clone()),
            ("where_is_four", args) => where_is_four(v, args.clone()),
            ("where_is_five", args) => where_is_five(v, args.clone()),
            ("where_is_six", args) => where_is_six(v, args.clone()),
            ("where_is_seven", args) => where_is_seven(v, args.clone()),
            ("where_is_eight", args) => where_is_eight(v, args.clone()),
            ("where_is_nine", args) => where_is_nine(v, args.clone()),
            ("where_is_same_color", args) => where_is_same_color(v, args.clone()),
            ("sum_all", args) => sum_all(v, args.clone()),
            ("even_count", args) => even_count(v, args.clone()),
            ("odd_count", args) => odd_count(v, args.clone()),
            ("big_three_sum", args) => big_three_sum(v, args.clone()),
            ("small_three_sum", args) => small_three_sum(v, args.clone()),
            ("where_is_continuous", args) => where_is_continuous(v, args.clone()),
            ("how_many_same_number", args) => how_many_same_number(v, args.clone()),
            ("max_diff_is", args) => max_diff_is(v, args.clone()),

            _ => todo!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cards::Cards;

    #[test]
    fn already_knew() {
        let q: Vec<Vec<Q>> = vec![
            vec![("what_is_your_answer", vec![1, 2, 3, 4, 1, 1, 1, 1])], // mine
            vec![("what_is_your_answer", vec![1, 2, 3, 4, 2, 2, 2, 2])],
            vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 1, 1, 1])],
            vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 2, 2, 2])],
        ];

        let expect = Cards::from_tuples(vec![(0, "red"), (0, "blue"), (9, "red"), (9, "blue")]);
        assert_eq!(solve(q), Ok(expect));
    }
}
