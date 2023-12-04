#[derive(Default)]
pub struct Card {
    winning: Vec<u8>,
    actual: Vec<u8>,
}

impl Card {
    pub fn parse(line: &str) -> Self {
        let mut line = line
            .bytes()
            .skip("Card 0".len())
            .skip_while(u8::is_ascii_whitespace)
            .skip_while(u8::is_ascii_digit)
            .skip(": ".len());

        let mut winning = Vec::with_capacity(10);

        loop {
            let word = line
                .by_ref()
                .skip_while(|&byte| byte == b' ')
                .take_while(|&byte| byte != b' ')
                .map(char::from)
                .collect::<String>();

            if word == "|" {
                break;
            }

            winning.push(word.parse().unwrap());
        }

        let mut actual = Vec::with_capacity(25);

        loop {
            let word = line
                .by_ref()
                .skip_while(|&byte| byte == b' ')
                .take_while(|&byte| byte != b' ')
                .map(char::from)
                .collect::<String>();

            if word.is_empty() {
                break;
            }

            actual.push(word.parse().unwrap());
        }

        Self { winning, actual }
    }

    pub fn count_winning_numbers(&self) -> usize {
        self.winning
            .iter()
            .filter(|number| self.actual.contains(number))
            .count()
    }
}
