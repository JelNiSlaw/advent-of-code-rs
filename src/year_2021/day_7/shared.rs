pub fn parse_crabs(line: &str) -> Vec<u16> {
    line.split(',')
        .map(|position| position.parse().unwrap())
        .collect::<Vec<_>>()
}
