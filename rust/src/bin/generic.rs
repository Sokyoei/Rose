/// Rust 泛型

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

impl Point<i32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt()
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut p1 = Point::new(1, 2);
    println!("p1: {:#?}", p1);
    println!("x: {}, y: {}", p1.get_x(), p1.get_y());
    println!("from origin: {}", p1.distance_from_origin());

    p1.swap();
    println!("swap p1: {:#?}", p1);

    let mut p2 = Point::new(1.1, 2.2);
    println!("p2: {:#?}", p2);
    p2.swap();
    println!("swap p2: {:#?}", p2);
}
