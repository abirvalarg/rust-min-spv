mod sys;

use crate::asm::svc;
use crate::rcc::AHB1Module;
use crate::gpio::PinMode;

#[no_mangle]
extern "C" fn _svcall(call: usize, a: usize, b: usize, c: usize) {
    let func = sys::calls[call];
    func(a, b, c);
}

enum Call {
    AHB1Enable = 0,
    PinMode = 1,
    PinWrite = 2
}

pub enum GpioMod {
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2
}

pub fn ahb1_enable(pos: AHB1Module) {
    unsafe {
        svc(Call::AHB1Enable as usize, pos as usize, 0, 0);
    }
}

pub fn pin_mode(gpio: GpioMod, pin: usize, mode: PinMode) {
    unsafe {
        svc(Call::PinMode as usize, gpio as usize, pin, mode as usize);
    }
}

pub fn pin_write(gpio: GpioMod, pin: usize, value: bool) {
    unsafe {
        svc(Call::PinWrite as usize, gpio as usize, pin, value as usize);
    }
}
