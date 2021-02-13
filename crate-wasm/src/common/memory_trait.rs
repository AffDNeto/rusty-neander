pub trait Memory {
    fn _read(&self, rem: usize) -> u8;
    fn _write(&mut self, rem: usize, value: u8);
    fn _increment_access_count(&mut self);
    fn get_access_count(&self) -> usize;
    fn reset_access_count(&mut self);

    fn direct_read(&mut self, rem: usize) -> u8 {
        let rdm = self._read(rem);
        self._increment_access_count();

        return rdm
    }
    fn indirect_read(&mut self, rem: usize) -> u8 {
        let mut address = self.direct_read(rem) as usize;
        return self.direct_read(address);
    }
    fn direct_write(&mut self, rem: usize, rdm: u8) {
        self._write(rem, value);
        self._increment_access_count();
    }
    fn indirect_write(&mut self, rem: usize, rdm: u8) {
        let address = self.direct_read(rem) as usize;
        self.direct_write(address, rdm);
    }
}