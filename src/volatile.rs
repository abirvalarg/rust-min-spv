extern "C" {
    pub fn read(addr: *const usize) -> usize;
    pub fn write(addr: *mut usize, value: usize);
}
