// os/src/main.rs
#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();               // begin addr of text segment
        fn etext();               // end addr of text segment
        fn srodata();             // start addr of Read-Only data segment
        fn erodata();             // end addr of Read-Only data ssegment
        fn sdata();               // start addr of data segment
        fn edata();               // end addr of data segment
        fn sbss();                // start addr of BSS segment
        fn ebss();                // end addr of BSS segment
        fn boot_stack();          // stack bottom
        fn boot_stack_top();      // stack top
    }
    clear_bss();
    println!("Hello, world!");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
