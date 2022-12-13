use std::cmp::Ordering;

use advent_of_code::Solve;

use super::shared::{right_order, Data};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "21423"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let marker_packets = [
            Data::List(vec![Data::List(vec![Data::Int(2)])]),
            Data::List(vec![Data::List(vec![Data::Int(6)])]),
        ];

        let mut packets = lines
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| Data::parse(line.strip_prefix('[').unwrap().strip_suffix(']').unwrap()))
            .chain(marker_packets.clone().into_iter())
            .collect::<Vec<_>>();

        packets.sort_unstable_by(|a, b| {
            if right_order(a, b).unwrap() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut first_packet = 0;
        let mut second_packet = 0;

        for (i, packet) in packets.into_iter().enumerate() {
            if packet == marker_packets[0] {
                first_packet = i + 1;
            } else if packet == marker_packets[1] {
                second_packet = i + 1;
            }
        }

        (first_packet * second_packet).to_string()
    }
}
