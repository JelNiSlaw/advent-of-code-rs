use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn parse(byte: u8, jokers: bool) -> Self {
        match byte {
            b'A' => Self::A,
            b'K' => Self::K,
            b'Q' => Self::Q,
            b'J' if jokers => Self::Joker,
            b'J' => Self::J,
            b'T' => Self::T,
            b'9' => Self::Nine,
            b'8' => Self::Eight,
            b'7' => Self::Seven,
            b'6' => Self::Six,
            b'5' => Self::Five,
            b'4' => Self::Four,
            b'3' => Self::Three,
            b'2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u32,
}

impl Hand {
    pub fn parse(line: &str, jokers: bool) -> Self {
        let mut line = line.bytes();
        let cards = line
            .by_ref()
            .take(5)
            .map(|byte| Card::parse(byte, jokers))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let bid = line
            .skip(" ".len())
            .map(char::from)
            .collect::<String>()
            .parse()
            .unwrap();

        Self { cards, bid }
    }

    pub fn r#type(&self) -> HandType {
        HandType::detect(self.cards.to_vec())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn detect(mut cards: Vec<Card>) -> Self {
        cards.retain(|card| !matches!(card, Card::Joker));
        cards.sort_unstable();

        let mut counts = cards
            .into_iter()
            .group_by(|&card| card)
            .into_iter()
            .map(|(_, group)| group.count())
            .collect::<Vec<_>>();

        counts.sort_unstable();

        match counts.as_slice() {
            [] | [1..=5] => Self::FiveOfAKind,
            [1, 1..=4] => Self::FourOfAKind,
            [2, 2..=3] => Self::FullHouse,
            [1, 1, 1..=3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 1..=2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!(),
        }
    }
}
