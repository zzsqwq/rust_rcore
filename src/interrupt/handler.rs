use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

use super::context::Context;
//use riscv::register::stvec;
//use riscv::register::scause::Scause;

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}
/*
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    panic!("Interrupted: {:?}", scause.cause());
}
*/
// 中断的处理入口
//
// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // 可以通过 Debug 来查看发生了什么中断
    // println!("{:x?}", context.scause.cause());
    // Experiment 2 直接检测stval 是否为0x0 即可
    if stval == 0x0 
    {
        println! ("SUCCESS!");
    }
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // Experiment 1
        Trap::Exception(Exception::LoadFault) => loadfault(context,stval),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

/// 处理 ebreak 断点
///
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    //Experiment 3 直接使context.sepc = 0 即可当sret时跳转到0x0 从而触发中断
    context.sepc = 0;
}
//  处理 LoadFault
fn loadfault(context: &mut Context, stval: usize)
{
    //Experiment 1
    panic!(
          "loadfault interrupt: context: {:x?}\nstval: {:x}",
          context,
          stval
      );
    
}
/// 处理时钟中断
///
/// 目前只会在 [`timer`] 模块中进行计数
fn supervisor_timer(_: &Context) {
    timer::tick();
}

/// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}
