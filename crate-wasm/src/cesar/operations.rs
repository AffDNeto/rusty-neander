use crate::cesar::ConditionFlags;

/// File with functions for operations done in the Cesar processor.
/// Keep in mind that all of them are function that receive the operands
/// and a processor flags reference returning the result and changing the flags properly

/// Compute flags based only on a value
/// set Z if the value is 0, clears otherwise
/// set N if the value is lower than 0, clears otherwise
fn compute_flags(value:u16, flags: &mut ConditionFlags){
    flags.n = (value & 0x8000) != 0;
    flags.z = value == 0;
}

// Returns 0 and sets flags with it, C and V are set to 0
pub(crate) fn clr(flags: &mut ConditionFlags ) -> u16 {
    flags.z = true;
    flags.n = false;
    flags.v= false;
    flags.c = false;
    return 0;
}

/// bitwise not operation, updates C to 1 e V to 0 always
pub(crate) fn not(a: u16, flags: &mut ConditionFlags) -> u16 {
    let result = !a;
    compute_flags(result, flags);
    flags.c = true;
    flags.v = false;

    return result;
}

/// Updates flags based on 'a', V and C are set to 0
pub(crate) fn tst(a: u16, flags: &mut ConditionFlags) -> u16 {
    compute_flags(a, flags);
    flags.c = false;
    flags.v = false;

    return a;
}

/// Rotates a bit to the right by one, completing the MSB with the carry flag
/// N: t, Z: t, V: (N xor C) , C: lsb
pub(crate) fn ror(a: u16, flags: &mut ConditionFlags) -> u16 {
    let result = (a >> 1) | if flags.c { 0x8000 } else { 0 };
    compute_flags(result, flags);
    flags.c = (result & 1) != 0 ;
    flags.v = flags.c ^ flags.n;

    return result;
}

/// Rotates a bit to the left by one, completing the LSB with the carry flag
/// N: t, Z: t, V: (N xor C) , C: msb
pub(crate) fn rol(a: u16, flags: &mut ConditionFlags) -> u16 {
    let result = (a << 1) | if flags.c { 1 } else { 0 };
    compute_flags(result, flags);
    flags.c = (result & 0x8000) != 0 ;
    flags.v = flags.c ^ flags.n;

    return result;
}

/// Arithmetic shifts a bit to the right by one, completing the MSB with 1
/// N: t, Z: t, V: (N xor C) , C: lsb
pub(crate) fn asr(a: u16, flags: &mut ConditionFlags) -> u16 {
    let result = (a >> 1) | 0x8000 ;
    compute_flags(result, flags);
    flags.c = (result & 1) != 0 ;
    flags.v = flags.c ^ flags.n;

    return result;
}

/// Arithmetic shifts a bit to the left by one, completing the LSB with 0
/// N: t, Z: t, V: (N xor C) , C: msb
pub(crate) fn asl(a: u16, flags: &mut ConditionFlags) -> u16 {
    let result = a << 1;
    compute_flags(result, flags);
    flags.c = (result & 0x8000) != 0 ;
    flags.v = flags.c ^ flags.n;

    return result;
}

/// Returns a+b IFF the flag carry is set
/// N: t, Z: t, V: t, C: t
pub(crate) fn adc(a: u16, flags: &mut ConditionFlags) -> u16 {
    let b = if flags.c { 1 } else { 0 };
    return add(a, b, flags);
}

/// Returns a+1
/// N: t, Z: t, V: t, C: t
pub(crate) fn inc(a: u16, flags: &mut ConditionFlags) -> u16 {
    return add(a, 1, flags);
}

/// Returns a+b
/// N: t, Z: t, V: t, C: t
pub(crate) fn add(a:u16, b:u16, flags: &mut ConditionFlags ) -> u16 {
    let s_a = a as i16;
    let s_b = b as i16;
    let (s_d, overflow) = s_a.overflowing_add(s_b);
    let d = s_d as u16;

    compute_flags(d, flags);
    flags.v = overflow;
    flags.c = (d < a) | (d < b);

    return d
}

/// Returns 0-a = -a
/// N: t, Z: t, V: t, C: t
pub(crate) fn neg(a: u16, flags: &mut ConditionFlags) -> u16 {
    let d = sub(0, a, flags);
    return d;
}

/// Returns a-1 IFF the flag carry is set
/// N: t, Z: t, V: t, C: t
pub(crate) fn sbc(a: u16, flags: &mut ConditionFlags) -> u16 {
    let b = if flags.c { 1 } else { 0 };
    let d = sub(a, b, flags);
    flags.c = !flags.c; // C flags behaviour is not the same as 'sub' per the processor description
    return d;
}

/// Returns a-1
/// N: t, Z: t, V: t, C: not(t)
pub(crate) fn dec(a: u16, flags: &mut ConditionFlags) -> u16 {
    return sub(a, 1, flags);
}

/// Returns a-b
/// N: t, Z: t, V: t, C: not(t)
pub(crate) fn sub(a:u16, b:u16, flags: &mut ConditionFlags ) -> u16 {
    let s_a = a as i16;
    let s_b = b as i16;
    let (s_d, overflow) = s_a.overflowing_sub(s_b);
    let d = s_d as u16;

    compute_flags(d, flags);
    flags.v = overflow;
    flags.c = !(b > a);

    return d
}

/// Updates the flags based on 'a' and sets V to 0, does nothing with C
pub(crate) fn mov(a: u16, flags: &mut ConditionFlags ) -> u16 {
    compute_flags(a, flags);
    flags.v = false;
    return a;
}

/// Compares a with b by doing a subtraction ( b- a ) and updating the flags
/// Nothing is returned;
pub(crate) fn cmp(a:u16, b:u16, flags: &mut ConditionFlags ) {
    sub(b, a, flags);
}

/// Bitwise AND operation between a and b
/// N: t, Z: t, V: 0, C: not changed
pub(crate) fn and(a:u16, b:u16, flags: &mut ConditionFlags ) -> u16 {
    let result = a & b;
    compute_flags(result, flags);
    flags.v = false;

    return result;
}

/// Bitwise OR operation between a and b
/// N: t, Z: t, V: 0, C: not changed
pub(crate) fn or(a:u16, b:u16, flags: &mut ConditionFlags ) -> u16 {
    let result = a | b;
    compute_flags(result, flags);
    flags.v = false;

    return result;
}