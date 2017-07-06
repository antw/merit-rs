extern crate test;

use participants::AlwaysOn;
use participants::Consumer;
use participants::Dispatchable;

/// Order contains information about the participants in the merit order.
pub struct Order {
    pub always_ons: Vec<AlwaysOn>,
    pub consumers: Vec<Consumer>,
    pub dispatchables: Vec<Dispatchable>,
    pub price_setters: Vec<Option<usize>>,
}

impl Order {
    pub fn new() -> Order {
        return Order {
            always_ons: vec![],
            consumers: vec![],
            dispatchables: vec![],
            price_setters: vec![None; 8760],
        };
    }

    /// demand_at returns the total demand for energy in frame.
    pub fn demand_at(&self, frame: usize) -> f64 {
        let mut sum = 0.0;

        for consumer in &self.consumers {
            sum += consumer.load_at(frame);
        }

        sum
    }

    /// Adds an always-on participant to the order.
    pub fn add_always_on(&mut self, ao: AlwaysOn) {
        self.always_ons.push(ao);
    }

    /// Adds a consumer to the order.
    pub fn add_consumer(&mut self, co: Consumer) {
        self.consumers.push(co);
    }

    /// Adds a dispatchable participant to the order.
    pub fn add_dispatchable(&mut self, di: Dispatchable) {
        self.dispatchables.push(di);
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    use participants::AlwaysOn;
    use participants::Consumer;
    use participants::Dispatchable;

    #[test]
    fn it_works() {
        let ao = AlwaysOn::new("ao".to_string(), vec![1.0; 8760], 1000.0);
        let co1 = Consumer::new("co".to_string(), vec![1.0; 8760], 2000.0);
        let co2 = Consumer::new("co".to_string(), vec![1.0; 8760], 200.0);
        let mut di = Dispatchable::new("disp1".to_string(), 10.0, 2.0, 3.0);

        let order = Order {
            always_ons: vec![ao],
            consumers: vec![co1, co2],
            dispatchables: vec![di],
            price_setters: vec![],
        };

        assert_eq!(order.demand_at(0), 2200.0);
    }
}
