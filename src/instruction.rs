use std::fmt;

use register::Register;
use address::Address;
use argument::Argument;


/// Represents a machine instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    HALT,
    SET(Register, Argument),
    PUSH(Argument),
    POP(Register),
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
    OUT(Argument), // XXX: this should take an argument, not a u8, per the example program
    IN(Address),
    NOOP
}

impl Instruction {

    /// The number of arguments a given opcode takes
    pub fn arg_count(opcode: u16) -> Option<usize> {
        match opcode {
            0 => Some(0),
            1 => Some(2),
            2 => Some(1),
            3 => Some(1),
            4 => Some(3),
            5 => Some(3),
            6 => Some(1),
            7 => Some(2),
            8 => Some(2),
            9 => Some(3),
            10 => Some(3),
            11 => Some(3),
            12 => Some(3),
            13 => Some(3),
            14 => Some(2),
            15 => Some(2),
            16 => Some(2),
            17 => Some(1),
            18 => Some(0),
            19 => Some(1),
            20 => Some(1),
            21 => Some(0),
            _ => None 
        }
    }


    /// Given an Instruction, produce it's opcode equivalent
    pub fn to_u16_sequence(self) -> Vec<u16> {
        match self {
            Instruction::HALT           => vec![0],
            Instruction::SET(r, a)      => vec![1, r.to_u16(), a.to_u16()],
            Instruction::PUSH(a)        => vec![2, a.to_u16()],
            Instruction::POP(r)         => vec![3, r.to_u16()],
            Instruction::EQ(r, a, b)    => vec![4, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::GT(r, a, b)    => vec![5, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::JMP(a)         => vec![6, a.to_u16()],
            Instruction::JT(a, b)       => vec![7, a.to_u16(), b.to_u16()],
            Instruction::JF(a, b)       => vec![8, a.to_u16(), b.to_u16()],
            Instruction::ADD(r, a, b)   => vec![9, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::MULT(r, a, b)  => vec![10, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::MOD(r, a, b)   => vec![11, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::AND(r, a, b)   => vec![12, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::OR(r, a, b)    => vec![13, r.to_u16(), a.to_u16(), b.to_u16()],
            Instruction::NOT(r, a)      => vec![14, r.to_u16(), a.to_u16()],
            Instruction::RMEM(r, a)     => vec![15, r.to_u16(), a.to_u16()],
            Instruction::WMEM(a, arg)   => vec![16, a.to_u16(), arg.to_u16()],
            Instruction::CALL(a)        => vec![17, a.to_u16()],
            Instruction::RET            => vec![18],
            Instruction::OUT(a)         => vec![19, a.to_u16()],
            Instruction::IN(a)          => vec![20, a.to_u16()],
            Instruction::NOOP           => vec![21]
        }
    }

    /// Given a sequence of 16b values, create an instruction. If given more than needed, remaining
    /// values are ignored.
    pub fn from_u16_sequence(seq: &Vec<u16>) -> Option<Instruction> {
        let opcode = seq[0];
        match opcode {
            0  => Some(Instruction::HALT),
            1  => Some(Instruction::SET(Register::new(seq[1]), Argument::new(seq[2]))),
            2  => Some(Instruction::PUSH(Argument::new(seq[1]))),
            3  => Some(Instruction::POP(Register::new(seq[1]))),
            4  => Some(Instruction::EQ(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            5  => Some(Instruction::GT(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            6  => Some(Instruction::JMP(Argument::new(seq[1]))),
            7  => Some(Instruction::JT(Argument::new(seq[1]), Argument::new(seq[2]))),
            8  => Some(Instruction::JF(Argument::new(seq[1]), Argument::new(seq[2]))),
            9  => Some(Instruction::ADD(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            10 => Some(Instruction::MULT(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            11 => Some(Instruction::MOD(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            12 => Some(Instruction::AND(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            13 => Some(Instruction::OR(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3]))),
            14 => Some(Instruction::NOT(Register::new(seq[1]), Argument::new(seq[2]))),
            15 => Some(Instruction::RMEM(Register::new(seq[1]), Address::new(seq[2]))),
            16 => Some(Instruction::WMEM(Address::new(seq[1]), Argument::new(seq[2]))),
            17 => Some(Instruction::CALL(Address::new(seq[1]))),
            18 => Some(Instruction::RET),
            19 => Some(Instruction::OUT(Argument::new(seq[1]))),
            20 => Some(Instruction::IN(Address::new(seq[1]))),
            21 => Some(Instruction::NOOP),
            _ => None
        }
    }


    pub fn name(self) -> &'static str {
        match self {
            Instruction::HALT           => "HALT",
            Instruction::SET(_, _)      => "SET",
            Instruction::PUSH(_)        => "PUSH",
            Instruction::POP(_)         => "POP",
            Instruction::EQ(_, _, _)    => "EQ",
            Instruction::GT(_, _, _)    => "GT",
            Instruction::JMP(_)         => "JMP",
            Instruction::JT(_, _)       => "JT",
            Instruction::JF(_, _)       => "JF",
            Instruction::ADD(_, _, _)   => "ADD",
            Instruction::MULT(_, _, _)  => "MULT",
            Instruction::MOD(_, _, _)   => "MOD",
            Instruction::AND(_, _, _)   => "AND",
            Instruction::OR(_, _, _)    => "OR",
            Instruction::NOT(_, _)      => "NOT",
            Instruction::RMEM(_, _)     => "RMEM",
            Instruction::WMEM(_, _)     => "WMEM",
            Instruction::CALL(_)        => "CALL",
            Instruction::RET            => "RET",
            Instruction::OUT(_)         => "OUT",
            Instruction::IN(_)          => "IN",
            Instruction::NOOP           => "NOOP"
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Instruction::HALT                      => write!(f, "HALT"),
            &Instruction::SET(ref r, ref a)         => write!(f, "SET {} {}", r, a),
            &Instruction::PUSH(ref a)               => write!(f, "PUSH {}", a),
            &Instruction::POP(ref r)                => write!(f, "POP {}", r),
            &Instruction::EQ(ref r, ref a, ref b)   => write!(f, "EQ {} {} {}", r, a, b),
            &Instruction::GT(ref r, ref a, ref b)   => write!(f, "GT {} {} {}", r, a, b),
            &Instruction::JMP(ref a)                => write!(f, "JMP {}", a),
            &Instruction::JT(ref a, ref b)          => write!(f, "JT {} {}", a, b),
            &Instruction::JF(ref a, ref b)          => write!(f, "JF {} {}", a, b),
            &Instruction::ADD(ref r, ref a, ref b)  => write!(f, "ADD {} {} {}", r, a, b),
            &Instruction::MULT(ref r, ref a, ref b) => write!(f, "MULT {} {} {}", r, a, b),
            &Instruction::MOD(ref r, ref a, ref b)  => write!(f, "MOD {} {} {}", r, a, b),
            &Instruction::AND(ref r, ref a, ref b)  => write!(f, "AND {} {} {}", r, a, b),
            &Instruction::OR(ref r, ref a, ref b)   => write!(f, "OR {} {} {}", r, a, b),
            &Instruction::NOT(ref r, ref a)         => write!(f, "NOT {} {}", r, a),
            &Instruction::RMEM(ref r, ref a)        => write!(f, "RMEM {} {}", r, a),
            &Instruction::WMEM(ref a, ref arg)      => write!(f, "WMEM {} {}", a, arg),
            &Instruction::CALL(ref a)               => write!(f, "CALL {}", a),
            &Instruction::RET                       => write!(f, "RET"),
            &Instruction::OUT(ref u)                => write!(f, "OUT {}", u),
            &Instruction::IN(ref a)                 => write!(f, "IN {}", a),
            &Instruction::NOOP                      => write!(f, "NOOP"),
        }
    }
}


#[cfg(test)]
mod tests {
    use constants::*;
    use super::*;

    mod display {
        use super::*;

        mod halt {
            use super::*;

            #[test]
            fn test() { 
                let s = format!("{}", Instruction::HALT);
                assert_eq!(s, "HALT");
            }
        }

        mod set {
            use super::*;

            #[test]
            fn lit() {
                let s = format!("{}", Instruction::SET(Register::R0, Argument::new(123)));
                assert_eq!(s, "SET R0 123");
            }

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::SET(Register::R0, Argument::new(REGISTER_1)));
                assert_eq!(s, "SET R0 R1");
            }
        }

        mod push {
            use super::*;

            #[test]
            fn lit() {
                let s = format!("{}", Instruction::PUSH( Argument::new(123)));
                assert_eq!(s, "PUSH 123");
            }

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::PUSH( Argument::new(REGISTER_1)));
                assert_eq!(s, "PUSH R1");
            }
        }

        mod pop {
            use super::*;

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::POP(Register::R0));
                assert_eq!(s, "POP R0");
            }
        }

        mod eq {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::EQ(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "EQ R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::EQ(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "EQ R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::EQ(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "EQ R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::EQ(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "EQ R0 R3 R5");
            }
        }

        mod gt {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::GT(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "GT R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::GT(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "GT R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::GT(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "GT R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::GT(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "GT R0 R3 R5");
            }
        }

        mod jmp {
            use super::*;

            #[test]
            fn lit() {
                let s = format!("{}", Instruction::JMP( Argument::new(123)));
                assert_eq!(s, "JMP 123");
            }

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::JMP( Argument::new(REGISTER_1)));
                assert_eq!(s, "JMP R1");
            }
        }

        mod jt {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::JT(Argument::new(456), Argument::new(123)));
                assert_eq!(s, "JT 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::JT(Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "JT 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::JT(Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "JT R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::JT(Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "JT R3 R5");
            }
        }

        mod jf {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::JF(Argument::new(456), Argument::new(123)));
                assert_eq!(s, "JF 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::JF(Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "JF 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::JF(Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "JF R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::JF(Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "JF R3 R5");
            }
        }

        mod add {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::ADD(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "ADD R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::ADD(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "ADD R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::ADD(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "ADD R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::ADD(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "ADD R0 R3 R5");
            }
        }

        mod mult {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::MULT(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "MULT R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::MULT(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "MULT R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::MULT(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "MULT R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::MULT(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "MULT R0 R3 R5");
            }
        }

        mod modulo {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::MOD(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "MOD R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::MOD(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "MOD R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::MOD(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "MOD R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::MOD(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "MOD R0 R3 R5");
            }
        }

        mod and {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::AND(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "AND R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::AND(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "AND R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::AND(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "AND R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::AND(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "AND R0 R3 R5");
            }
        }

        mod or {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = format!("{}", Instruction::OR(Register::R0, Argument::new(456), Argument::new(123)));
                assert_eq!(s, "OR R0 456 123");
            }

            #[test]
            fn lit_reg() {
                let s = format!("{}", Instruction::OR(Register::R0, Argument::new(456), Argument::new(REGISTER_1)));
                assert_eq!(s, "OR R0 456 R1");
            }

            #[test]
            fn reg_lit() {
                let s = format!("{}", Instruction::OR(Register::R0, Argument::new(REGISTER_7), Argument::new(123)));
                assert_eq!(s, "OR R0 R7 123");
            }

            #[test]
            fn reg_reg() {
                let s = format!("{}", Instruction::OR(Register::R0, Argument::new(REGISTER_3), Argument::new(REGISTER_5)));
                assert_eq!(s, "OR R0 R3 R5");
            }
        }

        mod not {
            use super::*;

            #[test]
            fn lit() {
                let s = format!("{}", Instruction::NOT(Register::R0, Argument::new(123)));
                assert_eq!(s, "NOT R0 123");
            }

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::NOT(Register::R0, Argument::new(REGISTER_1)));
                assert_eq!(s, "NOT R0 R1");
            }
        }

        mod rmem {
            use super::*;

            #[test]
            fn rmem() {
                let s = format!("{}", Instruction::RMEM(Register::R0, Address::new(123)));
                assert_eq!(s, "RMEM R0 @123");
            }

        }

        mod wmem {
            use super::*;

            #[test]
            fn lit() {
                let s = format!("{}", Instruction::WMEM(Address::new(1231), Argument::new(123)));
                assert_eq!(s, "WMEM @1231 123");
            }

            #[test]
            fn reg() {
                let s = format!("{}", Instruction::WMEM(Address::new(1231), Argument::new(REGISTER_1)));
                assert_eq!(s, "WMEM @1231 R1");
            }
        }

        mod call {
            use super::*;

            #[test]
            fn test() { 
                let s = format!("{}", Instruction::CALL(Address::new(1231)));
                assert_eq!(s, "CALL @1231");
            }
        }

        mod ret {
            use super::*;

            #[test]
            fn test() { 
                let s = format!("{}", Instruction::RET);
                assert_eq!(s, "RET");
            }
        }

        mod out {
            use super::*;

            #[test]
            fn lit() { 
                let s = format!("{}", Instruction::OUT(Argument::new(123)));
                assert_eq!(s, "OUT 123");
            }

            #[test]
            fn reg() { 
                let s = format!("{}", Instruction::OUT(Argument::new(REGISTER_1)));
                assert_eq!(s, "OUT R1");
            }
        }

        mod in_val {
            use super::*;

            #[test]
            fn test() { 
                let s = format!("{}", Instruction::IN(Address::new(123)));
                assert_eq!(s, "IN @123");
            }
        }

        mod noop {
            use super::*;

            #[test]
            fn test() { 
                let s = format!("{}", Instruction::NOOP);
                assert_eq!(s, "NOOP");
            }
        }
    }

    mod to_u16_sequence {
        use super::*;

        mod halt {
            use super::*;
            #[test]
            fn halt() {
                let h = Instruction::HALT;
                assert_eq!(h.to_u16_sequence(), vec![0]);
            }

        }

        mod set {
            use super::*;
            #[test]
            fn lit() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(123));
                assert_eq!(s.to_u16_sequence(), vec![1, REGISTER_0, 123]);
            }

            #[test]
            fn reg() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                assert_eq!(s.to_u16_sequence(), vec![1, REGISTER_0, REGISTER_1]);
            }
        }

        mod push {
            use super::*;
            #[test]
            fn lit() {
                let s = Instruction::PUSH(Argument::new(123));
                assert_eq!(s.to_u16_sequence(), vec![2, 123]);
            }

            #[test]
            fn reg() {
                let s = Instruction::PUSH(Argument::new(REGISTER_2));
                assert_eq!(s.to_u16_sequence(), vec![2, REGISTER_2]);
            }
        }

        mod pop {
            use super::*;
            #[test]
            fn reg() {
                let s = Instruction::POP(Register::new(REGISTER_2));
                assert_eq!(s.to_u16_sequence(), vec![3, REGISTER_2]);
            }
        }

        mod eq {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![4, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![4, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![4, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![4, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod gt {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![5, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![5, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![5, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![5, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod jmp {
            use super::*;

            #[test]
            fn reg() {
                let p = Instruction::JMP(Argument::new(123));
                let h = vec![6, 123];
                assert_eq!(p.to_u16_sequence(), h);
            }

            #[test]
            fn lit() {
                let p = Instruction::JMP(Argument::new(REGISTER_1));
                let h = vec![6, REGISTER_1];
                assert_eq!(p.to_u16_sequence(), h);
            }
        }

        mod jt {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::JT(Argument::new(123), Argument::new(456));
                let h = vec![7, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::JT(Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![7, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::JT(Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![7, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::JT(Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![7, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod jf {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::JF(Argument::new(123), Argument::new(456));
                let h = vec![8, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::JF(Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![8, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::JF(Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![8, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::JF(Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![8, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

        }

        mod add {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![9, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![9, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![9, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![9, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

        }

        mod mult {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![10, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![10, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![10, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![10, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod modulo { // used full name to avoid collision
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![11, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![11, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![11, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![11, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod and {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![12, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![12, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![12, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![12, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod or {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = vec![13, REGISTER_6, 123, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = vec![13, REGISTER_6, 123, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = vec![13, REGISTER_6, REGISTER_6, 456];
                assert_eq!(e.to_u16_sequence(), h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = vec![13, REGISTER_6, REGISTER_6, REGISTER_7];
                assert_eq!(e.to_u16_sequence(), h);
            }
        }

        mod not {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(123));
                assert_eq!(s.to_u16_sequence(), vec![14, REGISTER_0, 123]);
            }

            #[test]
            fn reg() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                assert_eq!(s.to_u16_sequence(), vec![14, REGISTER_0, REGISTER_1]);
            }

        }

        mod rmem {
            use super::*;

            #[test]
            fn rmem() {
                let s = Instruction::RMEM(Register::new(REGISTER_0), Address::new(123));
                assert_eq!(s.to_u16_sequence(), vec![15, REGISTER_0, 123]);
            }
        }

        mod wmem {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::WMEM(Address::new(123), Argument::new(456));
                assert_eq!(s.to_u16_sequence(), vec![16, 123, 456]);
            }


            #[test]
            fn reg() {
                let s = Instruction::WMEM(Address::new(123), Argument::new(REGISTER_0));
                assert_eq!(s.to_u16_sequence(), vec![16, 123, REGISTER_0]);
            }
        }

        mod call {
            use super::*;

            #[test]
            fn call() {
                let s = Instruction::CALL(Address::new(123));
                assert_eq!(s.to_u16_sequence(), vec![17, 123]);
            }
        }

        mod ret {
            use super::*;

            #[test]
            fn ret() {
                let h = Instruction::RET;
                assert_eq!(h.to_u16_sequence(), vec![18]);
            }

        }

        mod out {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::OUT(Argument::new(123));
                assert_eq!(s.to_u16_sequence(), vec![19, 123]);
            }

            #[test]
            fn reg() {
                let s = Instruction::OUT(Argument::new(REGISTER_0));
                assert_eq!(s.to_u16_sequence(), vec![19, REGISTER_0]);
            }
        }

        mod in_val { // to avoid reserved word
            use super::*;

            #[test]
            fn call() {
                let s = Instruction::IN(Address::new(123));
                assert_eq!(s.to_u16_sequence(), vec![20, 123]);
            }
        }

        mod noop {
            use super::*;

            #[test]
            fn noop() {
                let h = Instruction::NOOP;
                assert_eq!(h.to_u16_sequence(), vec![21]);
            }
        }
    }

    mod from_u16_sequence {
        use super::*;

        mod halt {
            use super::*;
            #[test]
            fn halt() {
                let h = Instruction::from_u16_sequence(&vec![0]).unwrap();
                assert_eq!(Instruction::HALT, h);
            }
        }

        mod set {
            use super::*;
            #[test]
            fn lit() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![1, REGISTER_0, 123]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![1, REGISTER_0, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }
        }

        mod push {
            use super::*;
            #[test]
            fn lit() {
                let p = Instruction::PUSH(Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![2, 123]).unwrap();
                assert_eq!(p, h);
            }

            #[test]
            fn reg() {
                let p = Instruction::PUSH(Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![2, REGISTER_1]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod pop {
            use super::*;
            #[test]
            fn reg() {
                let p = Instruction::POP(Register::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![3, REGISTER_1]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod eq {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod gt {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod jmp {
            use super::*;

            #[test]
            fn reg() {
                let p = Instruction::JMP(Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![6, 123]).unwrap();
                assert_eq!(p, h);
            }

            #[test]
            fn lit() {
                let p = Instruction::JMP(Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![6, REGISTER_1]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod jt {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = Instruction::JT(Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![7, 123, 456]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn lit_reg() {
                let s = Instruction::JT(Argument::new(123), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![7, 123, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg_lit() {
                let s = Instruction::JT(Argument::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![7, REGISTER_0, 123]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg_reg() {
                let s = Instruction::JT(Argument::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![7, REGISTER_0, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }
        }

        mod jf {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = Instruction::JF(Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![8, 123, 456]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn lit_reg() {
                let s = Instruction::JF(Argument::new(123), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![8, 123, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg_lit() {
                let s = Instruction::JF(Argument::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![8, REGISTER_0, 123]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg_reg() {
                let s = Instruction::JF(Argument::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![8, REGISTER_0, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }

        }

        mod add {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod mult {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

        }

        mod modulo { // full name to avoid collision
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod and {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod or {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, 123, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, 123, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, REGISTER_6, 456]).unwrap();
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, REGISTER_6, REGISTER_7]).unwrap();
                assert_eq!(e,h);
            }
        }

        mod not {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![14, REGISTER_0, 123]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![14, REGISTER_0, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }
        }

        mod rmem {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::RMEM(Register::new(REGISTER_0), Address::new(123));
                let h = Instruction::from_u16_sequence(&vec![15, REGISTER_0, 123]).unwrap();
                assert_eq!(s, h);
            }
        }

        mod wmem {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::WMEM(Address::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![16, 123, 456]).unwrap();
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::WMEM(Address::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![16, REGISTER_0, REGISTER_1]).unwrap();
                assert_eq!(s, h);
            }

        }

        mod call {
            use super::*;

            #[test]
            fn call() {
                let p = Instruction::CALL(Address::new(123));
                let h = Instruction::from_u16_sequence(&vec![17, 123]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod ret {
            use super::*;

            #[test]
            fn ret() {
                let h = Instruction::from_u16_sequence(&vec![18]).unwrap();
                assert_eq!(Instruction::RET, h);
            }
        }

        mod out {
            use super::*;

            #[test]
            fn lit() {
                let p = Instruction::OUT(Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![19, 123]).unwrap();
                assert_eq!(p, h);
            }

            #[test]
            fn reg() {
                let p = Instruction::OUT(Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![19, REGISTER_1]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod in_val { // to avoid reserved word
            use super::*;

            #[test]
            fn in_val() {
                let p = Instruction::IN(Address::new(123));
                let h = Instruction::from_u16_sequence(&vec![20, 123]).unwrap();
                assert_eq!(p, h);
            }
        }

        mod noop {
            use super::*;

            #[test]
            fn noop() {
                let h = Instruction::from_u16_sequence(&vec![21]).unwrap();
                assert_eq!(Instruction::NOOP, h);
            }
        }
    }
}
