

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
        // If the number in binary starts with zero it is a negative number
        self.set_negative(value.leading_ones() > 0);
    }

    fn add(&mut self, a: u8, b: u8) -> u8{
        // on unsigned operations an overflow means a carry in signed operations
        let result = a.wrapping_add(b);
        let carry = (result<a) | (result<b);
        let sa = a & 0x80;
        let sb = b & 0x80;
        let sresult = result & 0x80;
        // if a and b have the same signal and the result signal is different, then an overflow happened
        let overflow = (sa==sb) && (sa!=sresult);

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
    fn sub(&mut self, a:u8, b: u8) -> u8 {
        let result= a.wrapping_sub(b);
        let borrow = a < b;
        let sa = a & 0x80;
        let sb = b & 0x80;
        let sresult = result & 0x80;
        let overflow = (sa!=sb) && (sa!=sresult);

        self.set_borrow(borrow);
        self.set_overflow(overflow);
        self.compute_flags(result);
        return result;
    }

    fn neg(&mut self, a:u8) -> u8 {
        let result = (!a).wrapping_add(1);
        self.compute_flags(result);
        self.set_carry(a==0);
        return result;
    }
    
    /// Does does a shift left operation without setting any flags
    /// So it can be used by the rotate left final implementation. 
    fn _shl(&self, a: u8) -> (u8, bool) {
        let carry = ( a & 0b1000_0000) != 0;
        let result = a << 1;

        return (result, carry)
    }

    /// Does does a shift right operation without setting any flags
    /// So it can be used by the rotate right final implementation. 
    fn _shr(&self, a: u8) -> (u8, bool) {
        let carry = ( a & 0b0000_0001) != 0;
        let result = a >> 1;

        return (result, carry)
    }

    fn shl(&mut self, a: u8) -> u8 {
        let (result, carry) = self._shl(a);
        self.set_carry(carry);
        self.compute_flags(result);
        return result
    }

    fn shr(&mut self, a: u8) -> u8{
        let (result, carry) = self._shr(a);
        self.set_carry(carry);
        self.compute_flags(result);
        return result
    }

    fn rol(&mut self, a: u8) -> u8 {
        let (mut result, carry) = self._shl(a);
        
        if self.get_carry(){
            result |= 0b0000_0001;
        }

        self.set_carry(carry);
        self.compute_flags(result);
        return result
    }

    fn ror(&mut self, a: u8) -> u8{
        let (mut result, carry) = self._shr(a);
        
        if self.get_carry(){
            result |= 0b1000_0000;
        }

        self.set_carry(carry);
        self.compute_flags(result);
        return result
    }
}
