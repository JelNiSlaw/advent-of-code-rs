pub fn mix_numbers(numbers: impl Iterator<Item = i64>, rounds: u32) -> Vec<i64> {
    let mut numbers = numbers.enumerate().collect::<Vec<_>>();

    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let index = numbers
                .iter()
                .position(|&(position, _)| position == i)
                .unwrap();

            let (position, value) = numbers.remove(index);

            let new_index = (i64::try_from(index).unwrap() + value)
                .rem_euclid(numbers.len().try_into().unwrap());

            numbers.insert(new_index.try_into().unwrap(), (position, value));
        }
    }

    numbers.into_iter().map(|(_, value)| value).collect()
}

pub fn grove_coordinates(numbers: &[i64]) -> i64 {
    let zero_index = numbers.iter().position(|&value| value == 0).unwrap();

    [1000, 2000, 3000]
        .map(|offset| numbers[(zero_index + offset) % numbers.len()])
        .into_iter()
        .sum::<i64>()
}
