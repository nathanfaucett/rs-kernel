use memory_area::MemoryAreaIter;


pub struct Kernel {
    kernel_start_address: usize,
    kernel_end_address: usize,
    boot_start_address: usize,
    boot_end_address: usize,
    memory_areas: MemoryAreaIter
}

impl Kernel {
    pub fn new(
        kernel_start_address: usize,
        kernel_end_address: usize,
        boot_start_address: usize,
        boot_end_address: usize,
        memory_areas: MemoryAreaIter
    ) -> Self {
        Kernel {
            kernel_start_address: kernel_start_address,
            kernel_end_address: kernel_end_address,
            boot_start_address: boot_start_address,
            boot_end_address: boot_end_address,
            memory_areas: memory_areas
        }
    }

    pub fn get_kernel_start_address(&self) -> usize { self.kernel_start_address }
    pub fn get_kernel_end_address(&self) -> usize { self.kernel_end_address }

    pub fn get_boot_start_address(&self) -> usize { self.boot_start_address }
    pub fn get_boot_end_address(&self) -> usize { self.boot_end_address }

    pub fn get_memory_areas(&self) -> &MemoryAreaIter { &self.memory_areas }
    pub fn get_mut_memory_areas(&mut self) -> &mut MemoryAreaIter { &mut self.memory_areas }

    pub fn init(&self) -> &Self {
        self
    }
}
