use crate::common::register_trait::RegisterBank;

pub trait Memory {
    fn _read(&self, rem: usize) -> u8;
    fn _write(&mut self, rem: usize, value: u8);
    fn get_access_count(&self) -> usize;
    fn set_access_count(&mut self, value: usize);
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
    fn direct_write(&mut self) {
        self._write(self.get_rem() as usize, self.get_rdm());
        self._increment_access_count();
    }
}

pub trait MemoryAccess: Memory + RegisterBank{
    fn index_registrer_id(&self) -> u8;
    fn indirect_read(&mut self) {
        self.direct_read();
        self.set_rem(self.get_rdm());
        self.direct_read();
    }
    fn indirect_write(&mut self) {
        self.direct_read();
        self.set_rem(self.get_rdm());
        self.direct_write();
    }
    fn indexed_read(&mut self){ 
        let index = self.get_register(self.index_registrer_id());
        self.direct_read();
        self.set_rem(
            index.wrapping_add(self.get_rdm())
        );
        self.direct_read();
    }
    fn indexed_write(&mut self) { 
        let index = self.get_register(self.index_registrer_id());
        self.direct_read();
        self.set_rem(
            index.wrapping_add(self.get_rdm())
        );
        self.direct_write();
    }
    /// Value is already in the RDM register
    fn imediate_read(&mut self) { }
    fn imediate_write(&mut self) { self._write(self.get_rem() as usize, self.get_rdm())}
}