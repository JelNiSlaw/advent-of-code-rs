pub fn parse_fish(line: &str) -> [u64; 9] {
    let mut fish = [0; 9];

    for number in line.split(',') {
        fish[number.parse::<usize>().unwrap()] += 1;
    }

    fish
}

pub fn simulate_fish(mut fish: [u64; 9], days: u16) -> u64 {
    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.into_iter().sum()
}
