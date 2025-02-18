struct Person { name: String }

fn main() {
    let mut people = [
        Person { name: "John".to_string() },
        Person { name: "Jane".to_string() },
        Person { name: "Jim".to_string() },
    ];

    for person in &mut people {
        println!("Person: {}", person.name);
        person.name.push('!');
    }

    for person in &people {
        println!("Person: {}", person.name);
    }

    let mut person_iterator = people.iter_mut();
    while let Some(person) = person_iterator.next() {
        println!("Person: {}", person.name);
        person.name.push('!');
    }

    let numbers = [1, 2, 3, 4, 5];
    numbers.into_iter()
        .filter(|&number| number % 2 == 0)
        .filter(|&number| number > 3)
        .map(|number| number * 2)
        .for_each(|number| println!("Number: {}", number));
}
