#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p1 = Point::new(1, 2);
    println!("{:#?}", p1);
    let p2 = Point::new(1.1, 2.2);
    println!("{:#?}", p2);
}
