use itertools::Itertools;

pub fn parse_values(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .into_iter()
        .map(|line| {
            line.bytes()
                .group_by(|&byte| byte == b' ')
                .into_iter()
                .filter(|(is_space, _)| !is_space)
                .map(|(_, bytes)| bytes.map(char::from).collect::<String>().parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn differences(history: &[i32]) -> Vec<i32> {
    history
        .windows(2)
        .map(|values| values[1] - values[0])
        .collect::<Vec<i32>>()
}
