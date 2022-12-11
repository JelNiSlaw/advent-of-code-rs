use std::collections::{HashSet, VecDeque};

pub fn find_marker(input: &str, marker_size: usize) -> usize {
    let mut last_chars = VecDeque::with_capacity(marker_size);

    for (i, char) in input.chars().enumerate() {
        last_chars.push_back(char);

        if last_chars.len() >= marker_size {
            let current_size = last_chars.iter().collect::<HashSet<_>>().len();

            if current_size >= marker_size {
                return i + 1;
            }

            last_chars.pop_front();
        }
    }

    unreachable!()
}
