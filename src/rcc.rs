use crate::volatile::*;


#[repr(C)]
#[allow(non_snake_case)]
pub struct RCC {
    pub CR: usize,
    pub PLLCFGR: usize,
    pub CFGR: usize,
    pub CIR: usize,
    pub AHB1RSTR: usize,
    pub AHB2RSTR: usize,
    pub AHB3RSTR: usize,
    _res0: usize,
    pub APB1RSTR: usize,
    pub APB2RSTR: usize,
    _res1: usize,
    _res2: usize,
    pub AHB1ENR: usize,
    pub AHB2ENR: usize,
    pub AHB3ENR: usize,
    _res3: usize,
    pub APB1ENR: usize,
    pub APB2ENR: usize,
    _res4: usize,
    _res5: usize
}

#[allow(dead_code)]
pub enum AHB1Module {
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2
}

impl From<usize> for AHB1Module {
    fn from(module: usize) -> AHB1Module {
        match module {
            0 => AHB1Module::GPIOA,
            1 => AHB1Module::GPIOB,
            2 => AHB1Module::GPIOC,
            _ => panic!()
        }
    }
}

impl RCC {
    pub unsafe fn new(addr: usize) -> &'static mut RCC {
        &mut *(addr as *mut RCC)
    }

    pub fn enable_ahb1(&mut self, module: AHB1Module) {
        unsafe {
            write(&mut self.AHB1ENR, read(&self.AHB1ENR) | (1 << module as usize));
        }
    }
}
