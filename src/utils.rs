const BIG_LETTERS: &[(char, [&str; 6])] = &[
    ('A', [".##.", "#..#", "#..#", "####", "#..#", "#..#"]),
    ('E', ["####", "#...", "###.", "#...", "#...", "####"]),
    ('F', ["####", "#...", "###.", "#...", "#...", "#..."]),
    ('K', ["#..#", "#.#.", "##..", "#.#.", "#.#.", "#..#"]),
    ('L', ["#...", "#...", "#...", "#...", "#...", "####"]),
    ('P', ["###.", "#..#", "#..#", "###.", "#...", "#..."]),
    ('R', ["###.", "#..#", "#..#", "###.", "#.#.", "#..#"]),
    ('Z', ["####", "...#", "..#.", ".#..", "#...", "####"]),
];

pub fn decode_big_letters(text: &str) -> String {
    let mut lines = text.lines().map(|line| line.chars()).collect::<Vec<_>>();

    let mut result = String::new();

    loop {
        let letter = lines
            .iter_mut()
            .map(|line| line.by_ref().take(4).collect::<String>())
            .collect::<Vec<_>>();

        for line in &mut lines {
            line.by_ref().next();
        }

        if letter.iter().any(String::is_empty) {
            break;
        }

        result.push(decode_letter(&letter));
    }

    result
}

fn decode_letter(letter: &[String]) -> char {
    for &(char, big_letter) in BIG_LETTERS {
        if big_letter == letter {
            return char;
        }
    }

    panic!("unknown letter");
}
