struct Person {
    first_name: String,
    last_name: String,
}

enum HotelRoom {
    Vacant,
    Occupied(Person),
    Maintenance(Option<String>),
}

impl HotelRoom {
    fn get_description(&self) -> String {
        match self {
            HotelRoom::Vacant => "Vacant".to_string(),
            HotelRoom::Occupied(p) => format!("Occupied by {}", p.first_name),
            HotelRoom::Maintenance(_) => format!("In maintenance"),
        }
    }
}

fn main() {
    let room1 = HotelRoom::Vacant;
    let room2 = HotelRoom::Occupied(Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    });
    let room3 = HotelRoom::Maintenance(Some("Plumbing".to_string()));

    match room1 {
        HotelRoom::Vacant => println!("Room is vacant"),
        HotelRoom::Occupied(p) => println!("Room is occupied by {}", p.first_name),
        HotelRoom::Maintenance(reason) => println!("Room is in maintenance"),
    }

    let optional_number = Some(10);
    if optional_number.is_some() {
        println!("optional_number is some");
    }

    if let Some(n) = optional_number {
        println!("optional_number is {}", n);
    }
}
