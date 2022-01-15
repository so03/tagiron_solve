pub mod cards;
pub mod functions;

use cards::Cards;

use crate::functions::what_is_your_answer;

type Q = (&'static str, Vec<usize>);

fn main() {
    let q: Vec<Vec<Q>> = vec![
        vec![("what is your answer", vec![1, 2, 3, 4, 1, 1, 1, 1])], // mine
        vec![("what is your answer", vec![1, 2, 3, 4, 2, 2, 2, 2])],
        vec![("what is your answer", vec![5, 6, 7, 8, 3, 1, 1, 1])],
        vec![("what is your answer", vec![5, 6, 7, 8, 3, 2, 2, 2])],
    ];
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
        println!("Answer is decided.");
        println!("{:?}", ap);
    } else {
        println!("Answer is not decided.");
        println!("{:?}", ap);
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
        .filter(|cs| match &q {
            ("what is your answer", args) => what_is_your_answer(cs, args.clone()),
            _ => todo!(),
        })
        .collect()
}
