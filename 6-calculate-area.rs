enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}
fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    calculate_area(circle);
    calculate_area(square);
    calculate_area(rectangle);
}
fn calculate_area(shape:Shape) -> f64 {
    match shape {
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(a) => a * a,
    }
}
