const SNAFU: [char; 5] = ['=', '-', '0', '1', '2'];

pub fn from_snafu(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, char)| {
            #[allow(clippy::cast_possible_wrap)]
            let value = SNAFU.into_iter().position(|x| x == char).unwrap() as i64 - 2;
            value * (5_i64.pow(u32::try_from(i).unwrap()))
        })
        .sum()
}

pub fn to_snafu(mut number: i64) -> String {
    let mut snafu = Vec::new();

    while number != 0 {
        let char = (number + 2) % 5;
        snafu.push(SNAFU[usize::try_from(char).unwrap()]);
        number = (number + 2) / 5;
    }

    snafu.into_iter().rev().collect()
}
