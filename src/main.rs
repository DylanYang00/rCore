// os/src/main.rs
#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]


//use core::panic::PanicInfo;
//use core::fmt::{self, Write};
use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

//const SYSCALL_WRITE: usize = 64;
//const SYSCALL_EXIT: usize = 93;

/*
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}    

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
      syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}





*/

/*
#[no_mangle]
extern "C" fn _start(){
    //println!("Hello, world!");
    //sys_exit(9);
    shutdown();
}
*/


global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}    


fn clear_bss(){
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe { (a as *mut u8 ).write_volatile(0) }
    });
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{} {}",location.file(), location.line(), info.message().unwrap());
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}

