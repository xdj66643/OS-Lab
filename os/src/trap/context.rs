use riscv::register::sstatus::{Sstatus, self, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { 
        self.x[2] = sp;
    }
    
    // 专门为启动应用程序打造的“初始存档”
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User); // 告诉 CPU，我们要去 User 模式啦！
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // 把程序入口（0x80400000）设为下一条要执行的指令
        };
        cx.set_sp(sp); // 设置用户栈的栈顶
        cx
    }
}