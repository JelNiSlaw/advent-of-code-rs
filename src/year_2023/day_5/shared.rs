use std::ops::Range;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
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

    pub fn map_value(&self, source: u64) -> u64 {
        self.categories
            .iter()
            .fold(source, |source, category| category.map_value(source))
    }

    pub fn map_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        self.categories
            .iter()
            .fold(ranges, |ranges, category| category.map_ranges(ranges))
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

    fn map_value(&self, source: u64) -> u64 {
        self.maps
            .iter()
            .find_map(|map| map.map_value(source))
            .unwrap_or(source)
    }

    fn map_ranges(&self, mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut output = Vec::new();

        for map in &self.maps {
            let mut new_ranges = Vec::new();

            while let Some(range) = ranges.pop() {
                if range.start < map.source_range_start {
                    new_ranges.push(range.start..range.end.min(map.source_range_start));
                }

                if range.start < map.source_range_end() && range.end > map.source_range_start {
                    output.push(
                        range.start.max(map.source_range_start) - map.source_range_start
                            + map.destination_range_start
                            ..range.end.min(map.source_range_end()) - map.source_range_start
                                + map.destination_range_start,
                    );
                }

                if range.end > map.source_range_end() {
                    new_ranges.push(range.start.max(map.source_range_end())..range.end);
                }
            }

            ranges = new_ranges;
        }

        output.extend(ranges);

        output
    }
}

#[derive(Debug, Default)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
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

    const fn source_range_end(&self) -> u64 {
        self.source_range_start + self.range_length
    }

    const fn map_value(&self, source: u64) -> Option<u64> {
        if source >= self.source_range_start && source - self.source_range_start < self.range_length
        {
            Some(source - self.source_range_start + self.destination_range_start)
        } else {
            None
        }
    }
}
