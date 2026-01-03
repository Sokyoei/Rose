use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let now = Instant::now();
    println!("Now: {:?}", now);

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
