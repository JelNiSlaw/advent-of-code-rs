pub fn parse_line(pair: &str) -> [u32; 4] {
    let mut chars = pair.chars();
    let start_a = chars.by_ref().take_while(|&c| c != '-').collect::<String>();
    let end_a = chars.by_ref().take_while(|&c| c != ',').collect::<String>();
    let start_b = chars.by_ref().take_while(|&c| c != '-').collect::<String>();
    let end_b = chars.by_ref().collect::<String>();

    [
        start_a.parse().unwrap(),
        end_a.parse().unwrap(),
        start_b.parse().unwrap(),
        end_b.parse().unwrap(),
    ]
}
