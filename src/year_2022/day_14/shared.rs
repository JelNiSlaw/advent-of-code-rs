use std::collections::HashSet;

pub fn parse_line(line: &str) -> HashSet<(u32, u32)> {
    let pairs = line
        .split(" -> ")
        .map(|position| {
            let (x, y) = position.split_once(',').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut line_points = HashSet::new();

    for pair in pairs.windows(2) {
        let (x1, x2) = minmax(pair[0].0, pair[1].0);
        let (y1, y2) = minmax(pair[0].1, pair[1].1);
        for x in x1..=x2 {
            for y in y1..=y2 {
                line_points.insert((x, y));
            }
        }
    }

    line_points
}

fn minmax(a: u32, b: u32) -> (u32, u32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}
