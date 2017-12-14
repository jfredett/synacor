use u15::u15;
use register::Register;
use address::Address;
use constants::*;

#[derive(Debug, PartialEq, Eq)]
enum Argument {
    Literal(u15),
    Register(Register)
}

impl Argument {
    pub fn to_u15(self) -> u15 {
        match self {
            Argument::Literal(u) => u,
            Argument::Register(r) => r.to_u15()
        }
    }

    pub fn new(u: u15) -> Argument {
        if u >= u15(REGISTER_0) {
            return Argument::Register(Register::from_u15(u));
        } else {
            return Argument::Literal(u);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lit() {
        let arg = Argument::new(u15(123));
        assert_eq!(arg, Argument::Literal(u15(123)));
    }

    #[test]
    fn new_reg() {
        let arg = Argument::new(u15(REGISTER_0));
        assert_eq!(arg, Argument::Register(Register::R0));
    }

    #[test]
    #[should_panic]
    fn new_panic_on_out_of_range() {
        Argument::new(u15(REGISTER_7+1));
    }
}
