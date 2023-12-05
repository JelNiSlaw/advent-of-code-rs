#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    categories: Vec<Category>,
}

impl Almanac {
    pub fn parse(lines: Vec<String>) -> Self {
        let mut lines = lines.into_iter().peekable();

        let line = lines.next().unwrap();
        let mut line = line.bytes().skip("seeds: ".len()).peekable();

        let mut seeds = Vec::new();

        loop {
            let seed = line
                .by_ref()
                .take_while(u8::is_ascii_digit)
                .map(char::from)
                .collect::<String>()
                .parse()
                .unwrap();

            seeds.push(seed);

            if line.peek().is_none() {
                break;
            }
        }

        let mut categories = Vec::new();

        lines.next();

        loop {
            categories.push(Category::parse(lines.by_ref()));

            if lines.peek().is_none() {
                break;
            }
        }

        Self { seeds, categories }
    }

    pub fn map_value(&self, source: u32) -> u32 {
        self.categories
            .iter()
            .fold(source, |source, category| category.map_value(source))
    }

    pub fn reverse_map_value(&self, destination: u32) -> u32 {
        self.categories
            .iter()
            .rev()
            .fold(destination, |destination, category| {
                category.reverse_map_value(destination)
            })
    }
}

#[derive(Debug)]
struct Category {
    maps: Vec<Map>,
}

impl Category {
    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        lines.next();

        let mut maps = Vec::new();

        loop {
            let Some(line) = lines.next() else {
                break;
            };

            if line.is_empty() {
                break;
            }

            maps.push(Map::parse(&line));
        }

        Self { maps }
    }

    fn map_value(&self, source: u32) -> u32 {
        self.maps
            .iter()
            .find_map(|map| map.map_value(source))
            .unwrap_or(source)
    }

    fn reverse_map_value(&self, destination: u32) -> u32 {
        self.maps
            .iter()
            .find_map(|map| map.reverse_map_value(destination))
            .unwrap_or(destination)
    }
}

#[derive(Debug, Default)]
struct Map {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

impl Map {
    fn parse(line: &str) -> Self {
        let mut line = line.bytes();
        let mut map = Self::default();

        for i in 0..3 {
            let value = line
                .by_ref()
                .take_while(u8::is_ascii_digit)
                .map(char::from)
                .collect::<String>()
                .parse()
                .unwrap();

            *[
                &mut map.destination_range_start,
                &mut map.source_range_start,
                &mut map.range_length,
            ][i] = value;
        }

        map
    }

    const fn map_value(&self, source: u32) -> Option<u32> {
        if source >= self.source_range_start && source - self.source_range_start < self.range_length
        {
            Some(source - self.source_range_start + self.destination_range_start)
        } else {
            None
        }
    }

    const fn reverse_map_value(&self, destination: u32) -> Option<u32> {
        if destination >= self.destination_range_start
            && destination - self.destination_range_start < self.range_length
        {
            Some(destination - self.destination_range_start + self.source_range_start)
        } else {
            None
        }
    }
}
