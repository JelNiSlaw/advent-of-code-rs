use std::collections::VecDeque;

pub fn mix_numbers(numbers: impl Iterator<Item = i64>, rounds: usize) -> Vec<i64> {
    let mut numbers = numbers.enumerate().collect::<VecDeque<_>>();

    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let index = numbers
                .iter()
                .position(|&(position, _)| position == i)
                .unwrap();
            numbers.rotate_left(index);
            let (position, value) = numbers.pop_front().unwrap();
            #[allow(clippy::cast_possible_wrap)]
            let distance = value.rem_euclid(numbers.len() as i64);
            numbers.rotate_left(distance.try_into().unwrap());
            numbers.push_front((position, value));
        }
    }

    numbers.into_iter().map(|(_, number)| number).collect()
}

pub fn grove_coordinates(numbers: &[i64]) -> i64 {
    let zero_index = numbers.iter().position(|&x| x == 0).unwrap();

    [1_000, 2_000, 3_000]
        .into_iter()
        .map(|n| numbers[(zero_index + n) % numbers.len()])
        .sum()
}
