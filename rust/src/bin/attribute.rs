/// Rust 属性
///
/// #[...]  外部属性，作用于包含的单个代码项，比如 #[derive(Debug)]
/// #![...] 内部属性，作用于包含的 crate 或模块，比如 #![crate_name = "hello"]
///
/// 常用属性
/// 1、派生         #[derive(...)]
/// 2、条件编译     #[cfg(...)]
/// 3、警告、控制   #[warn(...)] #[allow(...)] #[deny(...)] #[forbid(...)]
/// 4、测试         #[test]
/// 5、内联         #[inline(...)]
/// 6、文档         #[doc(...)]

#[cfg(target_os = "windows")]
fn windows_only() {
    println!("windows only!");
}

#[cfg(target_os = "linux")]
fn linux_only() {
    println!("linux only!");
}

fn main() {
    #[cfg(target_os = "windows")]
    windows_only();
    #[cfg(target_os = "linux")]
    linux_only();

    println!("hello world!");
}
