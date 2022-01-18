fn main() {
    let q: Vec<Vec<tagiron_solve::Q>> = vec![
        vec![("what is your answer", vec![1, 2, 3, 4, 1, 1, 1, 1])], // mine
        vec![("what is your answer", vec![1, 2, 3, 4, 2, 2, 2, 2])],
        vec![("what is your answer", vec![5, 6, 7, 8, 3, 1, 1, 1])],
        vec![("what is your answer", vec![5, 6, 7, 8, 3, 2, 2, 2])],
    ];

    tagiron_solve::solve(q);
}
