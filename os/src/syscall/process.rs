use crate::batch::run_next_app;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    // 程序说它完事了，我们就去拉起下一个程序！
    run_next_app()
}