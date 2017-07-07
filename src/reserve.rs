pub struct Reserve {
    pub volume: f64,
    decay: Box<FnMut(usize, f64) -> f64>,
    store: Vec<Option<f64>>,
}

impl Reserve {
    /// Creates a new reserve with the provided `volume` and `decay`.
    pub fn new(volume: f64, decay: Box<FnMut(usize, f64) -> f64>) -> Reserve {
        let mut initial_store = vec![None; 8760];
        initial_store[0] = Some(0.0);

        Reserve {
            volume: volume,
            decay: decay,
            store: initial_store,
        }
    }

    /// Creates a new reserve with no decay using the provided `volume`.
    pub fn new_without_decay(volume: f64) -> Reserve {
        Reserve::new(volume, Box::new(|_, _| 0.0))
    }

    /// At returns how much energy is stored in the reserve at the end of the
    /// given frame. If the technology to which the reserve is attached is still
    /// being calculated, the energy stored may be subject to change.
    pub fn at(&mut self, frame: usize) -> f64 {
        match self.store[frame] {
            Some(amount) => { amount }
            None => {
                match self.store[frame - 1] {
                    Some(previous) => {
                        let amount = previous - self.decay_at(frame);
                        self.set(frame, amount);
                        amount
                    }
                    None => {
                        self.set(frame, 0.0);
                        0.0
                    }
                }
            }
        }
    }

    /// Set sets the amount in the reserve for the chosen frame. Ignores volume
    /// constraints and assumes you will check this yourself.
    pub fn set(&mut self, frame: usize, amount: f64) {
        self.store[frame] = Some(amount)
    }

    /// Add adds the given amount of energy in the chosen frame, ensuring that
    /// the amount stored does not exceed the volume of the reserve.
    ///
    /// Returns the amount of energy which was actually added; note that this
    /// may be less than the amount parameter.
    pub fn add(&mut self, frame: usize, amount: f64) -> f64 {
        let stored = self.at(frame);
        let mut assign = amount;

        if stored + amount > self.volume {
            assign = self.volume - stored;
        }

        self.set(frame, stored + assign);
        assign
    }

    /// Take removes the desired amount of energy from the reserve.
    ///
    /// Returns the amount of energy subtracted from the reserve. This may be
    /// less than asked for if insufficient was stored.
    pub fn take(&mut self, frame: usize, amount: f64) -> f64 {
        if amount < 0.0 {
            return 0.0;
        }

        let stored = self.at(frame);

        if stored > amount {
            self.set(frame, stored - amount);
            amount
        } else {
            self.set(frame, 0.0);
            stored
        }
    }

    /// DecayAt returns how much energy decayed in the reserve at the beginning
    /// of the chosen frame.
    pub fn decay_at(&mut self, frame: usize) -> f64 {
        if frame == 0 {
            return 0.0;
        }

        let stored = self.at(frame - 1);
        let decay = (self.decay)(frame, stored);

        if stored < decay {
            return stored;
        }

        return decay;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserve_assigns_volume_when_created() {
        let res = Reserve::new_without_decay(4.0);
        assert_eq!(res.volume, 4.0);
    }

    #[test]
    fn test_reserve_starts_empty() {
        let mut res = Reserve::new_without_decay(5.0);
        assert_eq!(res.at(0), 0.0);
    }

    #[test]
    fn test_reserve_adds_energy() {
        let mut res = Reserve::new_without_decay(5.0);
        res.add(0, 2.0);

        assert_eq!(res.at(0), 2.0);
    }

    #[test]
    fn test_reserve_carries_previous_values() {
        let mut res = Reserve::new_without_decay(5.0);
        res.add(0, 2.0);

        for frame in 1..3 {
            assert_eq!(res.at(frame), 2.0);
        }
    }

    #[test]
    fn test_reserve_volume_limit() {
        let mut res = Reserve::new_without_decay(2.0);

        let tests = [
            (1.0, 1.0, 1.0),
            (1.0, 1.0, 2.0),
            (1.0, 0.0, 2.0),
            (1.0, 0.0, 2.0),
        ];

        for test in tests.iter() {
            assert_eq!(res.add(0, test.0), test.1);
            assert_eq!(res.at(0), test.2);
        }
    }

    #[test]
    fn test_reserve_take() {
        let mut res = Reserve::new_without_decay(10.0);

        let tests = [
            (3.0, 3.0), // less than stored
            (5.0, 5.0), // full amount stored
            (7.0, 5.0), // more than stored
        ];

        for test in tests.iter() {
            res.set(0, 5.0);
            assert_eq!(res.take(0, test.0), test.1);
            assert_eq!(res.at(0), 5.0 - test.1);
        }
    }

    #[test]
    fn test_reserve_decay() {
        let mut res = Reserve::new(10.0, Box::new(|_, _| 2.0));
        res.add(0, 3.0);

        let tests = [
            (0, 0.0, 3.0), // frame, decay, stored
            (1, 2.0, 1.0),
            (2, 1.0, 0.0),
            (3, 0.0, 0.0),
        ];

        for test in tests.iter() {
            assert_eq!((test.0, res.decay_at(test.0)), (test.0, test.1));
            assert_eq!((test.0, res.at(test.0)), (test.0, test.2));
        }
    }
}
