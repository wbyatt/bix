#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

extern crate alloc;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    bix::hlt_loop();
}

// I don't understand why this has to be here, but it does.
pub fn hlt_loop() -> ! {
    bix::hlt_loop()
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
    use bix::allocator;
    use x86_64::VirtAddr;

    println!("Booting BIX...");
    bix::init();

    println!("Initializing heap...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("\theap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("\tvec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "\tcurrent reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "\tcurrent reference count is {}",
        Rc::strong_count(&cloned_reference)
    );

    println!("Heap initialized.");

    println!("BIX initialized, beginning halt loop...\n");

    print!("bix.kernel> ");
    bix::hlt_loop();
}
