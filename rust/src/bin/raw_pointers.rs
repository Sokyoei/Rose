fn main() {
    let value = 20;
    let ptr = &value;

    unsafe {
        println!("value: {}", *ptr);
    }
}
