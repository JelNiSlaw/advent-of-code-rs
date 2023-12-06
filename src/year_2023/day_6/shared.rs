pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn ways_of_winning(&self) -> u64 {
        let mut shortest_hold_time = None;
        let mut longest_hold_time = None;

        for hold_time in 1.. {
            if hold_time * (self.time - hold_time) > self.distance {
                shortest_hold_time = Some(hold_time);
                break;
            }
        }

        for negative_hold_time in 1.. {
            let hold_time = self.time - negative_hold_time;
            if hold_time * (self.time - hold_time) > self.distance {
                longest_hold_time = Some(hold_time);
                break;
            }
        }

        shortest_hold_time
            .unwrap()
            .abs_diff(longest_hold_time.unwrap())
            + 1
    }
}
