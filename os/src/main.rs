#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::{self, Write};

// === 1. 异常处理 ===
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// === 2. 系统调用底层 (Syscall) ===
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!("ecall",
             in("x10") args[0],
             in("x11") args[1],
             in("x12") args[2],
             in("x17") id,
             lateout("x10") ret
        );
    }
    ret
}

// === 3. 退出机制封装 ===
// 实现应用程序退出 [cite: 6, 7]
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// === 4. 输出支持封装 ===
// 封装对 SYSCALL_WRITE 的系统调用 [cite: 8]
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

// 实现基于 Write Trait 的数据结构 [cite: 8, 9]
struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// 实现 Rust 语言格式化宏 [cite: 10, 11, 12, 13]
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// === 5. 程序的真正入口 ===
// 满足老师的新要求，并调用退出机制 [cite: 7, 13]
#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world! 学号: 23301081");
    sys_exit(9);
}