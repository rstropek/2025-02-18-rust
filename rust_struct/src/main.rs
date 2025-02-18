#![allow(unused)]

struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    fn new(hours: u8, minutes: u8) -> Self {
        Self { hours, minutes }
    }

    fn high_noon() -> Self {
        Self { hours: 12, minutes: 0 }
    }

    fn get_minutes(&self) -> u8 {
        self.minutes
    }

    fn add_minutes(&self, minutes: u8) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }

    fn add_minutes_2(&mut self, minutes: u8) {
        self.minutes += minutes;
    }
}

fn main() {
    let mut clock = get_a_clock();
    print_clock(&clock);

    manipulate_clock(&mut clock);
    
    print_clock(&clock);
}

fn get_a_clock() -> Box<Clock> {
    let c = Box::new(Clock::high_noon());
    c
}

fn print_clock(clock: &Clock) {
    println!("{}", clock.hours);
}

fn manipulate_clock(clock: &mut Clock) {
    clock.hours = 13;
}
