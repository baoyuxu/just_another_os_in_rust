#![no_std]
#![no_main]
#![feature(naked_functions)]

// "import" PanicInfo from [core]
use core::panic::PanicInfo;

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start() -> ! {
    use core::arch::asm;
    asm!(
      // before we use the `la` pseudo-instruction for the first time,
      //  we need to set `gp` (google linker relaxation)
      ".option push",
      ".option norelax",
      "la gp, _global_pointer",
      ".option pop",

      // set the stack pointer
      "la sp, _init_stack_top",

      // "tail-call" to {entry} (call without saving a return address)
      "tail {entry}",
      entry = sym entry, // {entry} refers to the function [entry] below
      options(noreturn) // we must handle "returning" from assembly
    );
}

extern "C" fn entry() -> ! {
    use core::ptr::write_volatile;
    let addr = 0x1000_0000 as *mut u8;
    // Set data size to 8 bits.
    unsafe { write_volatile(addr.offset(3), 0b11) };
    // Enable FIFO.
    unsafe { write_volatile(addr.offset(2), 0b1) };
    // Enable receiver buffer interrupts.
    unsafe { write_volatile(addr.offset(1), 0b1) };

    // UART is now set up! Let's print a message.
    for byte in "Hello, world!\n".bytes() {
        unsafe { write_volatile(addr, byte) };
    }

    loop {}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
