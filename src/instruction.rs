use u15::u15;
use register::Register;
use address::Address;
use argument::Argument;

use constants::*;


/// Represents a machine instruction
#[derive(Debug, PartialEq, Eq)]
enum Instruction {
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
    OUT(u15),
    IN(Address),
    NOOP
}

impl Instruction {

    /// The number of arguments a given opcode takes
    pub fn arg_count(self) -> u8 {
        match self {
            Instruction::HALT          => 0,
            Instruction::SET(_, _)     => 2,
            Instruction::PUSH(_)       => 1,
            Instruction::POP(_)        => 1,
            Instruction::EQ(_, _, _)   => 3,
            Instruction::GT(_, _, _)   => 3,
            Instruction::JMP(_)        => 1,
            Instruction::JT(_, _)      => 2,
            Instruction::JF(_, _)      => 2,
            Instruction::ADD(_, _, _)  => 3,
            Instruction::MULT(_, _, _) => 3,
            Instruction::MOD(_, _, _)  => 3,
            Instruction::AND(_, _, _)  => 3,
            Instruction::OR(_, _, _)   => 3,
            Instruction::NOT(_, _)     => 2,
            Instruction::RMEM(_, _)    => 2,
            Instruction::WMEM(_, _)    => 2,
            Instruction::CALL(_)       => 1,
            Instruction::RET           => 0,
            Instruction::OUT(_)        => 1,
            Instruction::IN(_)         => 1,
            Instruction::NOOP          => 0
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
            //Instruction::CALL(a)        => vec![17, a.to_u16()],
            Instruction::RET            => vec![18],
            //Instruction::OUT(u)         => vec![19, u.0],
            //Instruction::IN(a)          => vec![20, a.to_u16()],
            Instruction::NOOP           => vec![21],
            _                           => vec![21]
        }
    }

    /// Given a sequence of 16b values, create an instruction. If given more than needed, remaining
    /// values are ignored.
    pub fn from_u16_sequence(seq: &Vec<u16>) -> Instruction {
        let opcode = seq[0];
        match opcode {
            0  => Instruction::HALT,
            1  => Instruction::SET(Register::new(seq[1]), Argument::new(seq[2])),
            2  => Instruction::PUSH(Argument::new(seq[1])),
            3  => Instruction::POP(Register::new(seq[1])),
            4  => Instruction::EQ(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            5  => Instruction::GT(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            6  => Instruction::JMP(Argument::new(seq[1])),
            7  => Instruction::JT(Argument::new(seq[1]), Argument::new(seq[2])),
            8  => Instruction::JF(Argument::new(seq[1]), Argument::new(seq[2])),
            9  => Instruction::ADD(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            10 => Instruction::MULT(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            11 => Instruction::MOD(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            12 => Instruction::AND(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            13 => Instruction::OR(Register::new(seq[1]), Argument::new(seq[2]), Argument::new(seq[3])),
            14 => Instruction::NOT(Register::new(seq[1]), Argument::new(seq[2])),
            15 => Instruction::RMEM(Register::new(seq[1]), Address::new(seq[2])),
            16 => Instruction::WMEM(Address::new(seq[1]), Argument::new(seq[2])),
            17 => { Instruction::NOOP },
            18 => Instruction::RET,
            19 => { Instruction::NOOP },
            20 => { Instruction::NOOP },
            21 => Instruction::NOOP,
            _  => { Instruction::NOOP }
        }
    }
}

// impl Display for Instruction
// ^-- should take an Instruction and produce an nicely formatted ASM-like thing, eg:
//
// HALT
// PUSH R7
// EQ R1 R7 R8
// JT 100
// JF 200
// etc

#[cfg(test)]
mod tests {
    use super::*;

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

        }

        mod in_val { // to avoid reserved word
            use super::*;

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
                let h = Instruction::from_u16_sequence(&vec![0]);
                assert_eq!(Instruction::HALT, h);
            }
        }

        mod set {
            use super::*;
            #[test]
            fn lit() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![1, REGISTER_0, 123]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::SET(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![1, REGISTER_0, REGISTER_1]);
                assert_eq!(s, h);
            }
        }

        mod push {
            use super::*;
            #[test]
            fn lit() {
                let p = Instruction::PUSH(Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![2, 123]);
                assert_eq!(p, h);
            }

            #[test]
            fn reg() {
                let p = Instruction::PUSH(Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![2, REGISTER_1]);
                assert_eq!(p, h);
            }
        }

        mod pop {
            use super::*;
            #[test]
            fn reg() {
                let p = Instruction::POP(Register::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![3, REGISTER_1]);
                assert_eq!(p, h);
            }
        }

        mod eq {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::EQ(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![4, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod gt {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::GT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![5, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod jmp {
            use super::*;

            #[test]
            fn reg() {
                let p = Instruction::JMP(Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![6, 123]);
                assert_eq!(p, h);
            }

            #[test]
            fn lit() {
                let p = Instruction::JMP(Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![6, REGISTER_1]);
                assert_eq!(p, h);
            }
        }

        mod jt {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = Instruction::JT(Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![7, 123, 456]);
                assert_eq!(s, h);
            }

            #[test]
            fn lit_reg() {
                let s = Instruction::JT(Argument::new(123), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![7, 123, REGISTER_1]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg_lit() {
                let s = Instruction::JT(Argument::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![7, REGISTER_0, 123]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg_reg() {
                let s = Instruction::JT(Argument::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![7, REGISTER_0, REGISTER_1]);
                assert_eq!(s, h);
            }
        }

        mod jf {
            use super::*;

            #[test]
            fn lit_lit() {
                let s = Instruction::JF(Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![8, 123, 456]);
                assert_eq!(s, h);
            }

            #[test]
            fn lit_reg() {
                let s = Instruction::JF(Argument::new(123), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![8, 123, REGISTER_1]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg_lit() {
                let s = Instruction::JF(Argument::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![8, REGISTER_0, 123]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg_reg() {
                let s = Instruction::JF(Argument::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![8, REGISTER_0, REGISTER_1]);
                assert_eq!(s, h);
            }

        }

        mod add {
            use super::*;
            #[test]
            fn lit_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::ADD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![9, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod mult {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MULT(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![10, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }

        }

        mod modulo { // full name to avoid collision
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::MOD(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![11, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod and {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::AND(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![12, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod or {
            use super::*;

            #[test]
            fn lit_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, 123, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn lit_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(123), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, 123, REGISTER_7]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_lit() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, REGISTER_6, 456]);
                assert_eq!(e,h);
            }

            #[test]
            fn reg_reg() {
                let e = Instruction::OR(Register::new(REGISTER_6), Argument::new(REGISTER_6), Argument::new(REGISTER_7));
                let h = Instruction::from_u16_sequence(&vec![13, REGISTER_6, REGISTER_6, REGISTER_7]);
                assert_eq!(e,h);
            }
        }

        mod not {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(123));
                let h = Instruction::from_u16_sequence(&vec![14, REGISTER_0, 123]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::NOT(Register::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![14, REGISTER_0, REGISTER_1]);
                assert_eq!(s, h);
            }
        }

        mod rmem {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::RMEM(Register::new(REGISTER_0), Address::new(123));
                let h = Instruction::from_u16_sequence(&vec![15, REGISTER_0, 123]);
                assert_eq!(s, h);
            }
        }

        mod wmem {
            use super::*;

            #[test]
            fn lit() {
                let s = Instruction::WMEM(Address::new(123), Argument::new(456));
                let h = Instruction::from_u16_sequence(&vec![16, 123, 456]);
                assert_eq!(s, h);
            }

            #[test]
            fn reg() {
                let s = Instruction::WMEM(Address::new(REGISTER_0), Argument::new(REGISTER_1));
                let h = Instruction::from_u16_sequence(&vec![16, REGISTER_0, REGISTER_1]);
                assert_eq!(s, h);
            }

        }

        mod call {
            use super::*;

        }

        mod ret {
            use super::*;

            #[test]
            fn ret() {
                let h = Instruction::from_u16_sequence(&vec![18]);
                assert_eq!(Instruction::RET, h);
            }
        }

        mod out {
            use super::*;

        }

        mod in_val { // to avoid reserved word
            use super::*;

        }

        mod noop {
            use super::*;

            #[test]
            fn noop() {
                let h = Instruction::from_u16_sequence(&vec![21]);
                assert_eq!(Instruction::NOOP, h);
            }
        }
    }
}
