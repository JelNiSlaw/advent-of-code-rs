pub fn get_directions(location: (usize, usize), forest: &[Vec<u32>]) -> [Vec<u32>; 4] {
    let (left, right) = {
        let (left, right) = forest[location.0].split_at(location.1);
        (
            left.to_vec(),
            right.iter().skip(1).copied().collect::<Vec<_>>(),
        )
    };

    let (up, down) = {
        let collumn = forest.iter().map(|row| row[location.1]).collect::<Vec<_>>();
        let (up, down) = collumn.split_at(location.0);
        (
            up.to_vec(),
            down.iter().skip(1).copied().collect::<Vec<_>>(),
        )
    };

    [left, right, up, down]
}
