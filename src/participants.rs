pub struct AlwaysOn {
    pub key: String,
    profile: Vec<f64>,
    total_production: f64,
}

impl AlwaysOn {
    pub fn new(
        key: String,
        profile: Vec<f64>,
        total_production: f64,
    ) -> AlwaysOn {
        AlwaysOn {
            key: key,
            profile: profile,
            total_production: total_production,
        }
    }

    pub fn load_at(&self, frame: usize) -> f64 {
        self.profile[frame] * self.total_production
    }
}

pub struct Consumer {
    pub key: String,
    profile: Vec<f64>,
    total_demand: f64,
}

impl Consumer {
    pub fn new(key: String, profile: Vec<f64>, total_demand: f64) -> Consumer {
        Consumer {
            key: key,
            profile: profile,
            total_demand: total_demand,
        }
    }

    pub fn load_at(&self, frame: usize) -> f64 {
        self.profile[frame] * self.total_demand
    }
}

pub struct Dispatchable {
    pub key: String,
    cost: f64,
    capacity: f64,
    units: f64,
    load: Vec<f64>,
}

impl Dispatchable {
    pub fn new(
        key: String,
        cost: f64,
        capacity: f64,
        units: f64,
    ) -> Dispatchable {
        Dispatchable {
            key: key,
            cost: cost,
            capacity: capacity,
            units: units,
            load: vec![0.0; 8760],
        }
    }

    pub fn total_capacity(&self) -> f64 {
        self.capacity * self.units
    }

    pub fn set_load_at(
        &mut self,
        frame: usize,
        amount: f64,
    ) -> Result<f64, &str> {
        if frame > self.load.capacity() - 1 {
            return Err("Cannot set load in frame");
        }

        self.load[frame] = amount;
        Ok(amount)
    }

    pub fn load_at(&self, frame: usize) -> f64 {
        self.load[frame]
    }
}

#[cfg(test)]
mod tests {
    use super::Dispatchable;
    use super::Consumer;

    #[test]
    fn consumer_works() {
        let cons = Consumer {
            key: "cons1".to_string(),
            profile: vec![0.5, 0.25, 0.0, 0.25],
            total_demand: 1000.0,
        };

        assert_eq!(cons.load_at(0), 500.0);
        assert_eq!(cons.load_at(1), 250.0);
        assert_eq!(cons.load_at(2), 0.0);
        assert_eq!(cons.load_at(3), 250.0);
    }

    #[test]
    fn dispatchable_works() {
        let mut disp = Dispatchable::new("disp1".to_string(), 10.0, 2.0, 3.0);

        assert_eq!(disp.key, "disp1");
        assert_eq!(disp.cost, 10.0);
        assert_eq!(disp.capacity, 2.0);
        assert_eq!(disp.units, 3.0);

        assert_eq!(disp.total_capacity(), 6.0);
        assert_eq!(disp.load_at(0), 0.0);

        match disp.set_load_at(0, 50.0) {
            Ok(val) => {
                assert_eq!(val, 50.0);
            }
            Err(msg) => {
                assert_eq!("", msg);
            }
        }

        assert_eq!(disp.load_at(0), 50.0);
    }
}
