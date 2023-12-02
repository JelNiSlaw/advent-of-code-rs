use std::iter::Peekable;

pub struct Game {
    pub subsets: Vec<Subset>,
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let mut line = line
            .bytes()
            .skip("Game 0".len())
            .skip_while(u8::is_ascii_digit)
            .skip(2)
            .peekable();

        let mut subsets = Vec::new();

        loop {
            subsets.push(Subset::parse(line.by_ref()));
            if line.peek().is_none() {
                break;
            }
        }

        Self { subsets }
    }
}

pub struct Subset {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Subset {
    pub fn parse(line: &mut Peekable<impl Iterator<Item = u8>>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        loop {
            let count = line
                .by_ref()
                .take_while(u8::is_ascii_digit)
                .map(char::from)
                .collect::<String>()
                .parse()
                .unwrap();

            match line.by_ref().next().unwrap() {
                b'r' => {
                    red = count;
                    line.by_ref().nth("ed".len() - 1);
                }
                b'g' => {
                    green = count;
                    line.by_ref().nth("reen".len() - 1);
                }
                b'b' => {
                    blue = count;
                    line.by_ref().nth("lue".len() - 1);
                }
                _ => unreachable!(),
            };

            let Some(&byte) = line.peek() else {
                break;
            };

            line.nth(1);

            if byte == b';' {
                break;
            }
        }

        Self { red, green, blue }
    }
}
