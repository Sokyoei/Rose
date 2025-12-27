use std::{cell::RefCell, rc::Rc, sync::Arc};

fn main() {
    // Box<T> 堆上分配内存
    let box_value = Box::new(10);
    println!("box_value: {}", box_value);

    // Rc<T>
    let rc_value = Rc::new(20);
    println!("rc_value: {}", rc_value);

    // Arc<T> 多处共享所有权
    let arc_value = Arc::new(30);
    println!("arc_value: {}", arc_value);

    // RefCell<T> 内部可变性
    let refcell_value = RefCell::new(40);
    println!("refcell_value: {}", refcell_value);

    // Arc<T> 线程安全的共享所有权

    // Mutex<T> 互斥访问数据

    // RwLock<T> 读取-写入访问数据

    // Weak<T> 解决循环引用
}
