#![no_std]
#![no_main]

#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod asm;
mod volatile;
mod rcc;
mod gpio;
mod spv;

#[no_mangle]
pub extern "C" fn start() {
    spv::ahb1_enable(rcc::AHB1Module::GPIOB);
    spv::pin_mode(spv::GpioMod::GPIOB, 6, gpio::PinMode::Output);
    let mut state = false;
    loop {
        state = !state;
        spv::pin_write(spv::GpioMod::GPIOB, 6, state);
        for _ in 0..1000000 {
            unsafe {
                asm::nop();
            }
        }
    }
}

#[no_mangle]
extern "C" fn _nmi() {
    panic!();
}

#[no_mangle]
extern "C" fn _hardfault() {
    panic!();
}
