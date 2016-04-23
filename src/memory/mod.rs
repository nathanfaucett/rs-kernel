

pub struct Memory {
    kernel_start: usize,
    kernel_end: usize,
    boot_start: usize,
    boot_end: usize,
}

impl Memory {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        boot_start: usize,
        boot_end: usize
    ) -> Self {
        Memory {
            kernel_start: kernel_start,
            kernel_end: kernel_end,
            boot_start: boot_start,
            boot_end: boot_end,
        }
    }
}
