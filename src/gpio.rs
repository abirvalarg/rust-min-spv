use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct GPIO {
    pub MODER: usize,
    pub OTYPER: usize,
    pub OSPEEDR: usize,
    pub PUPDR: usize,
    pub IDR: usize,
    pub ODR: usize,
    pub BSRR: usize,
    pub LCKR: usize,
    pub AFRL: usize,
    pub AFRH: usize
}

#[allow(dead_code)]
pub enum PinMode {
    Input = 0b00,
    Output = 0b01
}

impl From<usize> for PinMode {
    fn from(m: usize) -> PinMode {
        match m {
            0 => PinMode::Input,
            1 => PinMode::Output,
            _ => panic!()
        }
    }
}

impl GPIO {
    pub unsafe fn new(addr: usize) -> &'static mut GPIO {
        &mut *(addr as *mut GPIO)
    }

    pub fn pin_mode(&mut self, pin: u8, mode: PinMode) {
        let mut val = unsafe { read(&self.MODER) };
        val &= !(0b11usize << (pin * 2));
        val |= (mode as usize) << (pin * 2);
        unsafe { write(&mut self.MODER, val); }
    }

    pub fn write(&mut self, pin: u8, state: bool) {
        let mut val = unsafe { read(&self.ODR) };
        if state {
            val |= 1 << pin;
        } else {
            val &= !(1 << pin);
        }
        unsafe { write(&mut self.ODR, val); }
    }
}
