use rand::Rng;

fn main() {
    // Zufallszahl zwischen 1 und 10 generieren
    // 1..4 verliert
    // 5 gewinnt fast ;-)
    // 6..10 gewinnt

    // Array
    const MAX_RANDOM_NUMBERS: usize = 5;
    //let mut random_numbers = [0; MAX_RANDOM_NUMBERS];

    // Vector
    let mut random_numbers = Vec::with_capacity(MAX_RANDOM_NUMBERS);

    for _ in 0..MAX_RANDOM_NUMBERS {
        random_numbers.push(rand::rng().random_range(1..=10));
    }

    let mut results = Vec::with_capacity(MAX_RANDOM_NUMBERS);
    for i in 0..MAX_RANDOM_NUMBERS {
        let message = match random_numbers[i] {
            1..=4 => "verliert",
            5 => "gewinnt fast ;-)",
            6..=10 => "gewinnt",
            _ => panic!("Fehler: Zufallszahl außerhalb des gültigen Bereichs"),
        };
        results.push((random_numbers[i], message));
    }

    for (number, message) in results {
        println!("{}: {}", number, message);
    }

    // And now with iterators:
    (0..MAX_RANDOM_NUMBERS)
        .map(|_| rand::rng().random_range(1..=10))
        .map(|random_number| {
            (
                random_number,
                match random_number {
                    1..=4 => "verliert",
                    5 => "gewinnt fast ;-)",
                    6..=10 => "gewinnt",
                    _ => panic!("Fehler: Zufallszahl außerhalb des gültigen Bereichs"),
                },
            )
        })
        .for_each(|(number, message)| {
            println!("{}: {}", number, message);
        });
}
