#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Now were you rushing, or were  you dragging?");

    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!();
    serial_println!("running {} test(s)...", tests.len());
    for (idx, test) in tests.iter().enumerate() {
        test.run(idx);
    }

    exit_qemu(QemuExitCode::Success)
}

pub trait Testable {
    fn run(&self, idx: usize) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self, idx: usize) {
        serial_print!("\n{}. {} ... \t", idx+1, core::any::type_name::<T>());
        self();
        serial_println!("[bet]");
    }
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple ouput. Fails if printing to VGA Buffer panics");
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
