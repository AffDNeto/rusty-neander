pub trait Memory {
    fn _read(&self, rem: usize) -> u8;
    fn _write(&mut self, rem: usize, value: u8);
    fn get_access_count(&self) -> usize;
    fn set_access_count(&self, value: usize);
    fn get_rem(&self) -> u8;
    fn get_rdm(&self) -> u8;
    fn set_rem(&mut self, value: u8);
    fn set_rdm(&mut self, value: u8);
    
    fn _increment_access_count(&mut self){
        let inc = self.get_access_count() + 1;
        self.set_access_count(inc);
    }
    fn reset_access_count(&mut self) {
        self.set_access_count(0);
    }
    fn direct_read(&mut self) {
        let rdm = self._read(self.get_rem() as usize);
        self._increment_access_count();
        self.set_rdm(rdm)
    }
    fn indirect_read(&mut self) {
        self.direct_read();
        self.set_rem(self.get_rdm());
        self.direct_read(address);
    }
    fn direct_write(&mut self) {
        self._write(self.get_rem() as usize, self.get_rdm());
        self._increment_access_count();
    }
    fn indirect_write(&mut self) {
        self.direct_read(rem);
        self.direct_write(address, rdm);
    }
}