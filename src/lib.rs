#![no_std]


#[macro_use]
extern crate vga;
extern crate rlibc;


mod memory;
use memory::Memory;


pub struct Kernel {
    memory: Memory,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            memory: Memory::new(0, 0, 0, 0)
        }
    }

    pub fn init(&self) -> &Self {
        vga_println!("Hello, world!");
        self
    }
}

/* TODO: fix so we don't have to have this */
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop{}
}
