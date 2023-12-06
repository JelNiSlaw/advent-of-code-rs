pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn ways_of_winning(&self) -> u64 {
        let delta = self.time.pow(2) - 4 * self.distance;
        let delta_sqrt = (delta as f64).sqrt();

        let time = self.time as f64;
        let x1 = (time + delta_sqrt) / 2.;
        let x2 = (time - delta_sqrt) / 2.;

        (x1.ceil() - x2.floor()) as u64 - 1
    }
}
