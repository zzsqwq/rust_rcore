//禁止使用标准库
#![no_std]
//不使用main等全部Rust-level入口点作为程序的入口
#![no_main]

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
//! - `#![feature(global_asm)]`  
//!   内嵌整个汇编文件
#![feature(global_asm)]

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main -> !{
    loop{}
}
