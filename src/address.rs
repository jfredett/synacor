use std::str::FromStr;
use std::num::ParseIntError;

use register::Register;
use u15::u15;
use constants::*;

/// Represents a legal address in the VM
///
/// Legal addresses are defined by:
///
/// - 0..32767          literal value
/// - 32768..32775      registers 0..7
/// - 32776..65535      invalid
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Address(u16);

impl From<u8> for Address {
   fn from(small: u8) -> Address {
        Address(small as u16)
   }
}

impl From<u16> for Address {
   fn from(big: u16) -> Address {
        Address(big)
   }
}

impl FromStr for Address {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v_res = u16::from_str(s);
        return match v_res {
            Ok(v) => Ok(Address::from(v)),
            Err(e) => Err(e)
        }
    }
}

impl Address {
    pub fn new(u: u16) -> Address {
        return Address(u);
    }

    pub fn is_valid(&self) -> bool {
        return self.0 <= REGISTER_7;
    }

    pub fn is_invalid(&self) -> bool {
        return !self.is_valid();
    }

    pub fn is_register(&self) -> bool {
        return REGISTER_0 <= self.0 && self.0 <= REGISTER_7;
    }

    pub fn is_value(&self) -> bool {
        return self.is_valid() && !self.is_register();

    }

	/// If the Address is a register, return it, if it is not a register, return None.
    pub fn as_register(&self) -> Option<Register> {
        if self.is_register() {
            return Some(Register::new(self.0));
        } else {
            return None;
        }
    }

	pub fn value(&self) -> u16 { self.0 }
    pub fn to_u15(&self) -> u15 { u15(self.value()) }
}



#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn valid() {
		let a = Address::new(123);
		assert!(a.is_valid());
	}

	#[test]
	fn invalid() {
		let a = Address::new(40000);
		assert!(a.is_invalid());
	}

	#[test]
	fn register_some() {
		let a = Address::new(32770);
		let r = a.as_register();
		assert_eq!(r, Some(Register::R2));
	}

	#[test]
    fn register_none() {
		let a = Address::new(0);
		let r = a.as_register();
		assert_eq!(r, None);
    }
}



