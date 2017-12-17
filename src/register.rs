use std::fmt;

use address::Address;
use constants::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7
}

impl Register {
    pub fn new(u: u16) -> Register {
        let r_addr = u - REGISTER_0;
        if r_addr == 0 { return Register::R0; }
        else if r_addr == 1 { return Register::R1; }
        else if r_addr == 2 { return Register::R2; }
        else if r_addr == 3 { return Register::R3; }
        else if r_addr == 4 { return Register::R4; }
        else if r_addr == 5 { return Register::R5; }
        else if r_addr == 6 { return Register::R6; }
        else if r_addr == 7 { return Register::R7; }
        else { panic!("Invalid address for register"); }
    }

    pub fn as_address(&self) -> Address {
        return match *self {
            Register::R0 => Address::new(REGISTER_0),
            Register::R1 => Address::new(REGISTER_1),
            Register::R2 => Address::new(REGISTER_2),
            Register::R3 => Address::new(REGISTER_3),
            Register::R4 => Address::new(REGISTER_4),
            Register::R5 => Address::new(REGISTER_5),
            Register::R6 => Address::new(REGISTER_6),
            Register::R7 => Address::new(REGISTER_7)
        }
    }

    pub fn as_index(&self) -> usize {
        return match *self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7
        }
    }

    pub fn to_u16(&self) -> u16 {
        self.as_address().value()
    }
}


impl fmt::Display for Register {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            Register::R0 => write!(f, "R0"),
            Register::R1 => write!(f, "R1"),
            Register::R2 => write!(f, "R2"),
            Register::R3 => write!(f, "R3"),
            Register::R4 => write!(f, "R4"),
            Register::R5 => write!(f, "R5"),
            Register::R6 => write!(f, "R6"),
            Register::R7 => write!(f, "R7"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let r = Register::new(32772);
        assert_eq!(r, Register::R4);
    }

    #[test]
    #[should_panic]
    fn new_panics_with_lit_value() {
        let _ = Register::new(0);
    }

    #[test]
    #[should_panic]
    fn new_panics_with_invalid_value() {
        let _ = Register::new(42737);
    }
}
