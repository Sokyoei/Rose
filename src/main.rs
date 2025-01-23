use rose::get_project_dir;

#[warn(non_snake_case)]
fn main() {
    println!("Hello, world!");
    println!("ROOT: {}", get_project_dir().to_str().unwrap())
}
