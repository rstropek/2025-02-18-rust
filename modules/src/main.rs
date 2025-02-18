// Declare the modules
mod math;
mod vector;
mod vector3d;

use vector::Vector2d;
use vector3d::Vector3d;

fn main() {
    // Test the math operations
    let a = 10;
    let b = 5;

    println!("Math operations:");
    println!("{} + {} = {}", a, b, math::add(a, b));
    println!("{} - {} = {}", a, b, math::subtract(a, b));
    println!("{} * {} = {}", a, b, math::multiply(a, b));
    
    // Using match to handle the Option returned by divide
    match math::divide(a, b) {
        Some(result) => println!("{} / {} = {}", a, b, result),
        None => println!("Cannot divide by zero"),
    }

    // Test division by zero
    match math::divide(a, 0) {
        Some(result) => println!("{} / 0 = {}", a, result),
        None => println!("Cannot divide by zero"),
    }

    println!("\nVector2d operations:");
    // Create some 2D vectors
    let v1 = Vector2d::new(3.0, 4.0);
    let v2 = Vector2d::new(1.0, 2.0);

    println!("Vector2d 1: ({}, {})", v1.get_x(), v1.get_y());
    println!("Vector2d 2: ({}, {})", v2.get_x(), v2.get_y());
    println!("Length of vector2d 1: {}", v1.length());

    let v3 = v1.add(&v2);
    println!("Sum of 2D vectors: ({}, {})", v3.get_x(), v3.get_y());

    let v4 = v1.scale(2.0);
    println!("Vector2d 1 scaled by 2: ({}, {})", v4.get_x(), v4.get_y());

    println!("\nVector3d operations:");
    // Create some 3D vectors
    let v3d1 = Vector3d::new(1.0, 2.0, 3.0);
    let v3d2 = Vector3d::new(4.0, 5.0, 6.0);

    println!("Vector3d 1: ({}, {}, {})", v3d1.get_x(), v3d1.get_y(), v3d1.get_z());
    println!("Vector3d 2: ({}, {}, {})", v3d2.get_x(), v3d2.get_y(), v3d2.get_z());
    println!("Length of vector3d 1: {}", v3d1.length());

    let v3d3 = v3d1.add(&v3d2);
    println!("Sum of 3D vectors: ({}, {}, {})", 
        v3d3.get_x(), v3d3.get_y(), v3d3.get_z());

    let v3d4 = v3d1.scale(2.0);
    println!("Vector3d 1 scaled by 2: ({}, {}, {})", 
        v3d4.get_x(), v3d4.get_y(), v3d4.get_z());

    println!("Dot product: {}", v3d1.dot_product(&v3d2));
    
    let cross = v3d1.cross_product(&v3d2);
    println!("Cross product: ({}, {}, {})", 
        cross.get_x(), cross.get_y(), cross.get_z());
}
