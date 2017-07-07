#![feature(test)]

extern crate test;
extern crate rand;

mod calculate;
mod participants;
mod order;
mod reserve;

#[cfg(test)]
mod tests {
    use super::order::Order;
    use super::participants::*;
    use super::reserve::Reserve;
    use super::calculate::calculate;

    use test::Bencher;
    use rand;

    #[bench]
    fn bench_simple_calculation(b: &mut Bencher) {
        let mut demand = vec![0.0; 8760];
        let mut always = vec![0.0; 8760];

        for frame in 0..8760 {
            demand[frame] = rand::random::<f64>();
            always[frame] = rand::random::<f64>();
        }

        let mut order = Order::new();

        order.add_always_on(AlwaysOn::new("ao".to_string(), always, 3.0));
        order.add_consumer(Consumer::new("co".to_string(), demand, 1.5 * 40.0));

        for _ in 0..40 {
            order.add_dispatchable(
                Dispatchable::new("disp".to_string(), 0.0, 0.5, 3.0),
            );
        }

        b.iter(|| {
            // use `test::black_box` to prevent compiler optimizations from
            // disregarding unused values
            ::test::black_box(calculate(&mut order));
        });
    }

    #[bench]
    fn bench_reserve(b: &mut Bencher) {
        b.iter(|| {
            let mut res = Reserve::new(10.0, Box::new(|_, _| 0.2));
            res.set(0, 5.0);

            for frame in 0..8760 {
                ::test::black_box(res.add(frame, 1.2));
                ::test::black_box(res.take(frame, 1.0));
                ::test::black_box(res.at(frame));
            }
        });
    }
}
