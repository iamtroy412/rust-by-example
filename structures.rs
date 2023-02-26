// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Point {
        x: left_edge,
        y: top_edge,
    } = &rectangle.top_left;
    let Point {
        x: right_edge,
        y: bottom_edge,
    } = &rectangle.bottom_right;

    (right_edge - left_edge) * (top_edge - bottom_edge)
}

fn square(top_left: Point, v: f32) -> Rectangle {
    let bottom_right = Point {
        x: top_left.x + v,
        y: top_left.y - v,
    };
    Rectangle {
        top_left: top_left,
        bottom_right: bottom_right,
    }
}

fn main() {
    // Create struct field with init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point use a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    println!("area of {:?} is {}", rectangle, rect_area(&rectangle));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Deconstruct a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let new_point = Point { x: 2.0, y: 4.0 };
    let square = square(new_point, 2.0);
    println!("{:?}", square);
}
