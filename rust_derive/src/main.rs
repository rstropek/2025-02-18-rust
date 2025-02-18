#![allow(unused, dead_code)]

use std::{fmt::{Display, Formatter, Result}, ops::Add, str::FromStr};

#[derive(Copy, Clone, Debug)]
struct Vector2d {
    x: f32,
    y: f32,
}

impl Display for Vector2d {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}/{})", self.x, self.y)
    }
}

impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Vector2d) -> Vector2d {
        Vector2d { x: self.x + other.x, y: self.y + other.y }
    }
}

impl FromStr for Vector2d {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }
        let x: f32 = parts[0].parse().unwrap();
        let y: f32 = parts[1].parse().unwrap();
        Ok(Vector2d { x, y })
    }
}

fn main() {
    let mut vector = Vector2d { x: 1.0, y: 2.0 };
    let vector2 = vector;
    vector.x = 3.0;

    println!("{vector:?}");
    println!("{vector2}");

    let vector3 = vector + vector2;
    println!("{vector3}");

    let vector4: Vector2d = "1/2".parse().unwrap();
    println!("{vector4}");
}
