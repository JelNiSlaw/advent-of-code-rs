use std::collections::HashSet;
use std::u8;

pub struct Number {
    pub value: u16,
    pub x: u8,
    pub y: u8,
}

pub struct EngineSchematic {
    pub numbers: Vec<Number>,
    pub symbols: HashSet<(u8, u8)>,
}

impl EngineSchematic {
    pub fn parse(lines: &[String], parse_gears: bool) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = HashSet::new();

        for (line, y) in lines.iter().zip(0..) {
            let mut line = line.bytes().zip(0..).peekable();
            loop {
                match line.peek() {
                    Some((b'.', _)) => {
                        line.next();
                    }
                    Some(&(b'1'..=b'9', x)) => {
                        let mut value = String::with_capacity(3);
                        while line.peek().is_some_and(|(byte, _)| byte.is_ascii_digit()) {
                            value.push(char::from(line.next().unwrap().0));
                        }
                        numbers.push(Number {
                            value: value.parse().unwrap(),
                            x,
                            y,
                        });
                    }
                    Some(_) => {
                        let (byte, x) = line.next().unwrap();
                        if !parse_gears || byte == b'*' {
                            symbols.insert((x, y));
                        }
                    }
                    None => break,
                }
            }
        }

        Self { numbers, symbols }
    }
}

pub const fn decimal_len(mut value: u16) -> u8 {
    let mut len = 0;
    while value >= 10 {
        len += 1;
        value /= 10;
    }
    len
}
