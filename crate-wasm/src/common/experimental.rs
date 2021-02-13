

pub trait SimpleAlu {
    fn set_zero(&mut self, value:bool);
    fn set_negative(&mut self, value:bool);
    fn set_carry(&mut self, value:bool);
    fn set_overflow(&mut self, value:bool);
    fn set_borrow(&mut self, value:bool);
    fn get_zero(&self) -> bool;
    fn get_negative(&self) -> bool;
    fn get_carry(&self) -> bool;
    fn get_overflow(&self) -> bool;
    fn get_borrow(&self) -> bool;
    
    /// computes and sets the zero and negative flags, 
    /// since they can be computed independant of the operation
    fn compute_flags(&mut self, value: u8){
        self.set_zero(value == 0);
        /// If the number in binary starts with zero it is a negative number
        self.set_negative(value.leading_ones() > 0);
    }

    fn add(&mut self, a: u8, b: u8) -> u8{
        /// on unsigned operations an overflow means a carry in signed operations
        let (result, carry) = a.overflowing_add(value);
        let (_, overflow) = (a as i8).overflowing_add(value as i8);

        self.set_carry(carry);
        self.set_overflow(overflow);
        self.compute_flags(result);

        return result
    }

    fn and(&mut self, a: u8, b: u8) -> u8{
        let result = a & b;
        self.compute_flags(result);
        
        return result
    }
    fn or(&mut self, a: u8, b: u8) -> u8{
        let result = a | b;
        self.compute_flags(result);
        
        return result
    }
    fn not(&mut self, a: u8) -> u8{
        let result = !a;
        self.compute_flags(result);

        return result;
    }
}

pub trait ExtendedALU: SimpleAlu {
    fn sub(&self, a:u8, b: u8) -> u8 {
        let (result, borrow) = a.overflowing_sub(b);
        let (_, overflow) = (a as i8).overflowing_sub(b as i8);

        self.set_borrow(borrow);
        self.set_overflow(overflow);
        self.compute_flags(result);
        return result;
    }

    /// Does does a shift left operation without setting any flags
    /// So it can be used by the rotate left final implementation. 
    fn _shl(&self, a: u8) -> (u8, bool) {
        let carry = ( a & 0b1000_0000) == 1;
        let result = a << 1;

        return (result, carry)
    }

    /// Does does a shift right operation without setting any flags
    /// So it can be used by the rotate right final implementation. 
    fn _shr(&self, a: u8) -> (u8, bool) {
        let carry = ( a & 0b0000_0001) == 1;
        let result = a >> 1;

        return (result, carry)
    }

    fn shl(&mut self, a: u8) -> u8 {
        let (result, carry) = self._shl(a);
        self.set_carry(carry);
        return result
    }

    fn shr(&mut self, a: u8) -> u8{
        let (result, carry) = self._shr(a);
        self.set_carry(carry);
        return result
    }

    fn rol(&mut self, a: u8) -> u8 {
        let (mut result, carry) = self._shl(a);
        
        if self.get_carry(){
            result &= 0b0000_0001;
        }

        self.set_carry(carry);
        return result
    }

    fn ror(&mut self, a: u8) -> u8{
        let (mut result, carry) = self._shr(a);
        
        if self.get_carry(){
            result &= 0b1000_0000;
        }

        self.set_carry(carry);
        return result
    }
}

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