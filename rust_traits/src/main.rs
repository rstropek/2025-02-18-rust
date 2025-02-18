trait Billable {
    fn get_costs(&self) -> f64;
}

struct ConsultingWork {
    hours: f64,
    rate: f64,
}

impl Billable for ConsultingWork {
    fn get_costs(&self) -> f64 {
        self.hours * self.rate
    }
}

struct FixedPriceWork {
    price: f64,
}

impl Billable for FixedPriceWork {
    fn get_costs(&self) -> f64 {
        self.price
    }
}

impl Billable for f64 {
    fn get_costs(&self) -> f64 {
        *self
    }
}

fn bill(work: &dyn Billable) {
    println!("Costs: {}", work.get_costs());
}

fn main() {
    let work1 = ConsultingWork {
        hours: 10.0,
        rate: 100.0,
    };
    bill(&work1);

    let work2 = FixedPriceWork {
        price: 1000.0,
    };
    bill(&work2);

    let work3 = 1000.0;
    bill(&work3);

    let works: Vec<&dyn Billable> = vec![&work1, &work2, &work3];
    for work in works {
        bill(work);
    }
}
