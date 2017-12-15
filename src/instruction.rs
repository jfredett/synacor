use u15::u15;
use register::Register;
use address::Address;
use argument::Argument;

use constants::*;


#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    HALT,
    SET(Register, Argument),
    PUSH(Argument),
    POP(Argument),
    EQ(Register, Argument, Argument),
    GT(Register, Argument, Argument),
    JMP(Argument),
    JT(Argument, Argument),
    JF(Argument, Argument),
    ADD(Register, Argument, Argument),
    MULT(Register, Argument, Argument),
    MOD(Register, Argument, Argument),
    AND(Register, Argument, Argument),
    OR(Register, Argument, Argument),
    NOT(Register, Argument),
    RMEM(Register, Address),
    WMEM(Address, Argument),
    CALL(Address),
    RET,
    OUT(u15),
    IN(Address),
    NOOP
}

impl Instruction {

    /// the number of arguments a given opcode takes
    pub fn arg_count(self) -> u8 {
        match self {
            Instruction::HALT          => 0,
            Instruction::SET(r, a)     => 2,
            Instruction::PUSH(a)       => 1,
            Instruction::POP(r)        => 1,
            Instruction::EQ(r, a, b)   => 3,
            Instruction::GT(r, a, b)   => 3,
            Instruction::JMP(a)        => 1,
            Instruction::JT(a,  b)     => 2,
            Instruction::JF(a, b)      => 2,
            Instruction::ADD(r, a, b)  => 3,
            Instruction::MULT(r, a, b) => 3,
            Instruction::MOD(r, a, b)  => 3,
            Instruction::AND(r, a, b)  => 3,
            Instruction::OR(r, a, b)   => 3,
            Instruction::NOT(r, a)     => 2,
            Instruction::RMEM(r, a)    => 2,
            Instruction::WMEM(a, arg)  => 2,
            Instruction::CALL(a)       => 1,
            Instruction::RET           => 0,
            Instruction::OUT(u)        => 1,
            Instruction::IN(a)         => 1,
            Instruction::NOOP          => 0
        }
    }

    pub fn to_u16_sequence(self) -> Vec<u16> {
        match self {
            Instruction::HALT                         => vec![0],
            Instruction::SET(r, a)                    => vec![1, r.to_u16(), a.to_u16()],
            Instruction::PUSH(a)                      => vec![2, a.to_u16()],
            Instruction::POP(r)                       => vec![3, r.to_u16()],
            //&Instruction::EQ(ref r, ref a, ref b)   => vec![4, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::GT(ref r, ref a, ref b)   => vec![5, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::JMP(ref a)                => vec![6, a.to_u16()],
            //&Instruction::JT(ref a, ref  b)         => vec![7, a.to_u16(), b.to_u16()],
            //&Instruction::JF(ref a, ref b)          => vec![8, a.to_u16(), b.to_u16()],
            //&Instruction::ADD(ref r, ref a, ref b)  => vec![9, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::MULT(ref r, ref a, ref b) => vec![10, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::MOD(ref r, ref a, ref b)  => vec![11, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::AND(ref r, ref a, ref b)  => vec![12, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::OR(ref r, ref a, ref b)   => vec![13, r.to_u16(), a.to_u16(), b.to_u16()],
            //&Instruction::NOT(ref r, ref a)         => vec![14, r.to_u16(), a.to_u16()],
            //&Instruction::RMEM(ref r, ref a)        => vec![15, r.to_u16(), a.to_u16()],
            //&Instruction::WMEM(a, arg)              => vec![16, a.to_u16(), arg.to_u16()],
            //&Instruction::CALL(ref a)               => vec![17, a.to_u16()],
            //&Instruction::RET                       => vec![18],
            //&Instruction::OUT(ref u)                => vec![19, u.0],
            //&Instruction::IN(ref a)                 => vec![20, a.to_u16()],
            //&Instruction::NOOP                      => vec![21],
            _                                         => vec![21]
        }
    }

    pub fn from_u16_sequence(seq: &Vec<u16>) -> Instruction {
        let opcode = seq[0];
        match opcode {
            0  => Instruction::HALT,
            1  => Instruction::SET(Register::new(seq[1]), Argument::new(seq[2])),
            2  => Instruction::PUSH(Argument::new(seq[1])),
            3  => Instruction::POP(Argument::new(seq[1])),
            4  => { Instruction::NOOP },
            5  => { Instruction::NOOP },
            6  => { Instruction::NOOP },
            7  => { Instruction::NOOP },
            8  => { Instruction::NOOP },
            9  => { Instruction::NOOP },
            10 => { Instruction::NOOP },
            11 => { Instruction::NOOP },
            12 => { Instruction::NOOP },
            13 => { Instruction::NOOP },
            14 => { Instruction::NOOP },
            15 => { Instruction::NOOP },
            16 => { Instruction::NOOP },
            17 => { Instruction::NOOP },
            18 => { Instruction::NOOP },
            19 => { Instruction::NOOP },
            20 => { Instruction::NOOP },
            21 => { Instruction::NOOP },
            _  => { Instruction::NOOP }
        }
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   mod to_u16_sequence {
       use super::*;

       #[test]
       fn halt() {
            let h = Instruction::HALT;
            assert_eq!(h.to_u16_sequence(), vec![0]);
       }

       #[test]
       fn set_literal() {
            let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(123));
            assert_eq!(s.to_u16_sequence(), vec![1, 32768, 123]);
       }

       #[test]
       fn set_register() {
            let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(REGISTER_1));
            assert_eq!(s.to_u16_sequence(), vec![1, 32768, 32769]);
       }

       #[test]
       fn push_lit() {
            let s = Instruction::PUSH(Argument::new(123));
            assert_eq!(s.to_u16_sequence(), vec![2, 123]);
       }

       #[test]
       fn push_reg() {
            let s = Instruction::PUSH(Argument::new(REGISTER_2));
            assert_eq!(s.to_u16_sequence(), vec![2, 32770]);
       }

       #[test]
       fn pop_lit() {
            let s = Instruction::POP(Argument::new(123));
            assert_eq!(s.to_u16_sequence(), vec![3, 123]);
       }

       #[test]
       fn pop_reg() {
            let s = Instruction::POP(Argument::new(REGISTER_2));
            assert_eq!(s.to_u16_sequence(), vec![3, 32770]);
       }
   }

   mod from_u16_sequence {
       use super::*;

       #[test]
       fn halt() {
            let h = Instruction::from_u16_sequence(&vec![0]);
            assert_eq!(Instruction::HALT, h);
       }

       #[test]
       fn set_lit() {
            let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(123));
            let h = Instruction::from_u16_sequence(&vec![1, 32768, 123]);
            assert_eq!(s, h);
       }

       #[test]
       fn set_reg() {
            let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(REGISTER_1));
            let h = Instruction::from_u16_sequence(&vec![1, 32768, 32769]);
            assert_eq!(s, h);
       }

       #[test]
       fn push_lit() {
            let p = Instruction::PUSH(Argument::new(123));
            let h = Instruction::from_u16_sequence(&vec![2, 123]);
            assert_eq!(p, h);
       }

       #[test]
       fn push_reg() {
            let p = Instruction::PUSH(Argument::new(REGISTER_1));
            let h = Instruction::from_u16_sequence(&vec![2, 32769]);
            assert_eq!(p, h);
       }

       #[test]
       fn pop_lit() {
            let p = Instruction::POP(Argument::new(123));
            let h = Instruction::from_u16_sequence(&vec![3, 123]);
            assert_eq!(p, h);
       }

       #[test]
       fn pop_reg() {
            let p = Instruction::POP(Argument::new(REGISTER_1));
            let h = Instruction::from_u16_sequence(&vec![3, 32769]);
            assert_eq!(p, h);
       }
   }
}
