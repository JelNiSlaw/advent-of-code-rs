pub fn parse_sensors(lines: impl Iterator<Item = String>) -> Vec<((i32, i32), u32)> {
    lines
        .map(|line| {
            let mut chars = line.chars().skip("Sensor at ".len());

            let sensor = parse_x_y(chars.by_ref());
            let beacon = parse_x_y(chars.by_ref().skip(" closest beacon is at ".len()));

            let x_diff = sensor.0.abs_diff(beacon.0);
            let y_diff = sensor.1.abs_diff(beacon.1);

            (sensor, x_diff + y_diff)
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
