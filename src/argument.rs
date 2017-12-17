use std::fmt;

use u15::u15;
use register::Register;
use constants::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Argument {
    Literal(u15),
    Register(Register)
}

impl Argument {
    pub fn to_u16(self) -> u16 {
        match self {
            Argument::Literal(u) => u.0,
            Argument::Register(r) => r.to_u16()
        }
    }

    pub fn new(u: u16) -> Argument {
        if u >= REGISTER_0 {
            return Argument::Register(Register::new(u));
        } else {
            return Argument::Literal(u15(u));
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            &Argument::Literal(ref u) => write!(f, "{}", u),
            &Argument::Register(ref r) => write!(f, "{}", r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lit() {
        let arg = Argument::new(123);
        assert_eq!(arg, Argument::Literal(u15(123)));
    }

    #[test]
    fn new_reg() {
        let arg = Argument::new(REGISTER_0);
        assert_eq!(arg, Argument::Register(Register::R0));
    }

    #[test]
    #[should_panic]
    fn new_panic_on_out_of_range() {
        Argument::new(REGISTER_7+1);
    }
}
