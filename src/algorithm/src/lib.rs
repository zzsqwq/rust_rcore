#![no_std]

//extern crate alloc; //申明使用外部的alloc crate
use alloc;
mod allocator; //引入allocator
pub use allocator::*;
