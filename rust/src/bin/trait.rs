/// Rust trait 特征
///
///

trait Speakable {
    fn speak(&self) -> String;
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

impl Speakable for Person {
    fn speak(&self) -> String {
        format!("I'm {}, {} year old", self.name, self.age)
    }
}

#[derive(Debug)]
struct Fox {
    kind: String,
    age: i32,
}

impl Speakable for Fox {
    fn speak(&self) -> String {
        format!("I'm a {} fox, {} year old", self.kind, self.age)
    }
}

fn main() {
    let ahri = Person::new(String::from("Ahri"), 13);
    println!("p: {:?}", ahri);
    println!("p say: {}", ahri.speak());

    let nono = Fox {
        kind: String::from("red"),
        age: 3,
    };
    println!("fox: {:?}", nono);
    println!("fox say: {}", nono.speak());
}
