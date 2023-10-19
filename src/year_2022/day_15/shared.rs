pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub range: u32,
}

pub fn parse_sensors(lines: impl Iterator<Item = String>) -> Vec<Sensor> {
    lines
        .map(|line| {
            let mut chars = line.chars().skip("Sensor at ".len());

            let (sensor_x, sensor_y) = parse_x_y(chars.by_ref());
            let (beacon_x, beacon_y) =
                parse_x_y(chars.by_ref().skip(" closest beacon is at ".len()));

            let x_diff = sensor_x.abs_diff(beacon_x);
            let y_diff = sensor_y.abs_diff(beacon_y);

            Sensor {
                x: sensor_x,
                y: sensor_y,
                range: x_diff + y_diff,
            }
        })
        .collect()
}

fn parse_x_y(mut chars: impl Iterator<Item = char>) -> (i32, i32) {
    let x = chars
        .by_ref()
        .skip("x=".len())
        .take_while(|&c| c == '-' || c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    let y = chars
        .by_ref()
        .skip(" y=".len())
        .take_while(|&c| c == '-' || c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    (x, y)
}
