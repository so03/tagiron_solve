fn main() {
    let q: Vec<Vec<tagiron_solve::Q>> = vec![
        vec![("what_is_your_answer", vec![1, 2, 3, 4, 1, 1, 1, 1])], // mine
        vec![("what_is_your_answer", vec![1, 2, 3, 4, 2, 2, 2, 2])],
        vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 1, 1, 1])],
        vec![("what_is_your_answer", vec![5, 6, 7, 8, 3, 2, 2, 2])],
    ];

    println!("{:?}", tagiron_solve::solve(q));
}
