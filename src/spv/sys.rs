use crate::rcc;
use crate::gpio;

static mut RCC: Option<&'static mut rcc::RCC> = None;

static mut GPIO: Option<[&'static mut gpio::GPIO; 3]> = None;

#[no_mangle]
unsafe extern "C" fn __init_rust() {
    RCC = Some(rcc::RCC::new(0x4002_3800));
    GPIO = Some([
        0x4002_0000,
        0x4002_0400,
        0x4002_0800
    ].map(|a| gpio::GPIO::new(a)));
}

pub const calls: [fn(usize, usize, usize); 3] = [
    ahb1_enable,
    pin_mode,
    pin_write
];

fn ahb1_enable(pos: usize, _: usize, _: usize) {
    let rcc = unsafe { RCC.take().unwrap() };
    rcc.enable_ahb1(pos.into());
    unsafe {
        RCC = Some(rcc);
    }
}

fn pin_mode(gpio: usize, pin: usize, mode: usize) {
    let mut gpios = unsafe { GPIO.take().unwrap() };
    let gpio = &mut gpios[gpio];
    gpio.pin_mode(pin as u8, mode.into());
    unsafe {
        GPIO = Some(gpios);
    }
}

fn pin_write(gpio: usize, pin: usize, value: usize) {
    let mut gpios = unsafe { GPIO.take().unwrap() };
    let gpio = &mut gpios[gpio];
    gpio.write(pin as u8, value != 0);
    unsafe {
        GPIO = Some(gpios);
    }
}
