#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)] // 💡 适配 2026 版编译器
fn main() -> i32 {
    println!("Into Test store_fault, we will insert an invalid store operation...");
    println!("Kernel should kill this application!");
    // 故意往内存地址 0 的地方写数据，这在用户态是不被允许的！
    unsafe { (0x0 as *mut u8).write_volatile(0); }
    0
}