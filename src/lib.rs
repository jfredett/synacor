#![feature(try_from, const_fn)]

mod constants {
    pub const REGISTER_0 : u16 = 32768;
    pub const REGISTER_1 : u16 = 32769;
    pub const REGISTER_2 : u16 = 32770;
    pub const REGISTER_3 : u16 = 32771;
    pub const REGISTER_4 : u16 = 32772;
    pub const REGISTER_5 : u16 = 32773;
    pub const REGISTER_6 : u16 = 32774;
    pub const REGISTER_7 : u16 = 32775;

    pub const MODULUS : u16 = 32768;

    pub const U15_MAX : u16 = MODULUS - 1;
    pub const U15_MIN : u16 = 0;
}

pub mod address;
pub mod register;
pub mod u15;
pub mod instruction;
pub mod argument;
pub mod binary;
pub mod vm;

