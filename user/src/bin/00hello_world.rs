#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)] // 💡 适配 2026 版编译器
fn main() -> i32 {
    println!("Hello, world! 学号: 23301081");
    unsafe {
        asm!("sret");
    }
    0
}