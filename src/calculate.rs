use order::Order;

pub fn calculate(order: &mut Order) {
    // TODO sort dispatchables

    for frame in 0..8760 {
        calculate_frame(frame, order);
    }
}

fn calculate_frame(frame: usize, order: &mut Order) {
    let mut remaining = order.demand_at(frame);

    for producer in order.always_ons.iter() {
        remaining -= producer.load_at(frame);
    }

    for (index, producer) in order.dispatchables.iter_mut().enumerate() {
        let max_load = producer.total_capacity();

        if max_load < remaining {
            producer.set_load_at(frame, max_load).unwrap();
        } else {
            // remaining is less than 0 if always-on supply exceeds demand.
            if remaining > 0.0 {
                producer.set_load_at(frame, remaining).unwrap();
            }

            order.price_setters[frame] = Some(index);
            break;
        }

        remaining -= max_load;
    }
}
