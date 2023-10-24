fn add(a: i32, b: i32) -> i32 {
    let c = {
        let d = 1;
        d + 1
    };

    fn nono(n: i32) -> i32 {
        return n;
    }

    a + b + c + nono(4)
}

fn ahri() {
    let a = 80;
    if a >= 90 {
        println!("A")
    } else if a >= 70 && a < 90 {
        println!("B")
    } else {
        println!("C")
    }
}

fn main() {
    let _tuple: (i32, f64, String, &str) = (10, 1.23, "hello".to_string(), "world");
    println!("value: {}", _tuple.0);

    let mut _list: [i32; 5] = [1, 2, 3, 4, 5];
    for i in _list.iter() {
        println!("value: {}", i);
    }
    for i in 0..5 {
        println!("value: {}", _list[i]);
    }

    let mut i = 0;
    let num = loop {
        let n = _list[i];
        if n == 3 {
            break n;
        }
        i += 1;
    };
    println!("{}", num.to_string());

    println!("add(): {}", add(1, 2));
    hello();
    ahri();
}

fn hello() {
    println!("hello world");
}
