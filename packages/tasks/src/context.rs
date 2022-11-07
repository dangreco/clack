#![allow(dead_code)]

use core::arch::asm;

#[inline(always)]
pub fn save_context() {
  unsafe { asm!("") }
}

#[inline(always)]
pub fn restore_context() {
  unsafe { asm!("") }
}
