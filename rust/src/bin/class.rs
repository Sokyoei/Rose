#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }

    fn print_info(&self) {
        println!("I'm {}, {} year old", self.name, self.age)
    }
}

fn main() {
    let p = Person::new(String::from("Ahri"), 13);
    p.print_info();
    println!("p: {:?}", p);
}
