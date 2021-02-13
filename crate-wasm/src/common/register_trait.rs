

pub trait RegisterBank {
    fn get_pc(&self) -> u8;
    fn get_ri(&self) -> u8;
    fn get_rem(&self) -> u8;
    fn get_rdm(&self) -> u8;
    fn get_register(&self, id: u8) -> u8;

    fn set_pc(&mut self, value: u8);
    fn set_ri(&mut self, value: u8);
    fn set_rem(&mut self, value: u8);
    fn set_rdm(&mut self, value: u8);
    fn set_register(&mut self, id: u8, value: u8);

    fn increment_pc(&mut self) -> u8 {
        let n_pc = self.get_pc().wrapping_add(1);
        self.set_pc(n_pc);
        return n_pc;
    }
}