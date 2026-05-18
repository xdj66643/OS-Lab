#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod batch;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S")); // 💡 把那 3 个应用程序加载进来的汇编文件

fn clear_bss() {
    // 💡 适配 2026 编译器
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    let sbss_ptr = sbss as *const () as usize;
    let ebss_ptr = ebss as *const () as usize;
    (sbss_ptr..ebss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[unsafe(no_mangle)] // 💡 适配 2026 编译器
pub fn rust_main() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");
    
    // 初始化异常捕获机制，把 __alltraps 的地址告诉 CPU
    trap::init();
    
    // 初始化批处理系统，识别挂载进来的 3 个应用程序
    batch::init();
    
    // 开始运行第一个应用程序！
    batch::run_next_app();
}