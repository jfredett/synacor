use std::ops::*;
use constants::*;

/// a type representing the weird 15b modular number system.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub struct u15(pub u16);


impl u15 {
    pub const fn min_value() -> u15 {
        return u15(0);
    }

    pub const fn max_value() -> u15 {
        return u15(MODULUS - 1);
    }
}

impl Not for u15 {
    type Output = u15;

    // mask out the high bit so it's a 15b value.
    #[inline]
    fn not(self) -> u15 { u15((!self.0) % MODULUS) }
}

/// implements binop traits for u15
/// _all math is performed modulo 32768_
macro_rules! binop_trait {
    ($trait:ident, $method:ident) => {
        binop_trait!($trait, $method, u15, true);
    };
    ($trait:ident, $method:ident, $T:ty) => {
        impl $trait<$T> for u15 {
            type Output = u15;

            #[inline]
            fn $method(self, rhs: $T) -> u15 { u15($trait::$method(self.0, rhs) % MODULUS) }
        }
    };
    // this is a crappy hack to unwrap the newtype, but whatever.
    ($trait:ident, $method:ident, $T:ty, $with_zero_call:ident) => {
        impl $trait<$T> for u15 {
            type Output = u15;

            #[inline]
            fn $method(self, rhs: $T) -> u15 { u15($trait::$method(self.0, rhs.0) % MODULUS) }
        }
    }

}

/// implements binop assign traits for u15
macro_rules! binop_assign_trait {
    ($trait:ident, $method:ident) => {
        binop_assign_trait!($trait, $method, u15, true);
    };
    ($trait:ident, $method:ident, $T:ty) => {
        impl $trait<$T> for u15 {
            #[inline]
            fn $method(&mut self, rhs: $T) { $trait::$method(&mut self.0, rhs); }
        }
    };
    // this is a crappy hack to unwrap the newtype, but whatever.
    ($trait:ident, $method:ident, $T:ty, $with_zero_call:ident) => {
        impl $trait<$T> for u15 {
            #[inline]
            fn $method(&mut self, rhs: $T) { $trait::$method(&mut self.0, rhs.0); }
        }
    }
}

binop_trait!(BitOr, bitor);
binop_trait!(BitAnd, bitand);
binop_trait!(BitXor, bitxor);

binop_assign_trait!(BitOrAssign, bitor_assign);
binop_assign_trait!(BitXorAssign, bitxor_assign);
binop_assign_trait!(BitAndAssign, bitand_assign);

binop_trait!(Add, add);
binop_trait!(Mul, mul);
binop_trait!(Sub, sub);
binop_trait!(Div, div);
binop_trait!(Rem, rem);

binop_assign_trait!(AddAssign, add_assign);
binop_assign_trait!(MulAssign, mul_assign);
binop_assign_trait!(SubAssign, sub_assign);
binop_assign_trait!(DivAssign, div_assign);
binop_assign_trait!(RemAssign, rem_assign);

binop_trait!(Shl, shl);
binop_trait!(Shl, shl, i8);
binop_trait!(Shl, shl, i16);
binop_trait!(Shl, shl, i32);
binop_trait!(Shl, shl, i64);
binop_trait!(Shl, shl, u8);
binop_trait!(Shl, shl, u16);
binop_trait!(Shl, shl, u32);
binop_trait!(Shl, shl, u64);
binop_trait!(Shr, shr);
binop_trait!(Shr, shr, i8);
binop_trait!(Shr, shr, i16);
binop_trait!(Shr, shr, i32);
binop_trait!(Shr, shr, i64);
binop_trait!(Shr, shr, u8);
binop_trait!(Shr, shr, u16);
binop_trait!(Shr, shr, u32);
binop_trait!(Shr, shr, u64);

binop_assign_trait!(ShlAssign, shl_assign);
binop_assign_trait!(ShrAssign, shr_assign);
binop_assign_trait!(ShlAssign, shl_assign, i8);
binop_assign_trait!(ShlAssign, shl_assign, i16);
binop_assign_trait!(ShlAssign, shl_assign, i32);
binop_assign_trait!(ShlAssign, shl_assign, i64);
binop_assign_trait!(ShlAssign, shl_assign, u8);
binop_assign_trait!(ShlAssign, shl_assign, u16);
binop_assign_trait!(ShlAssign, shl_assign, u32);
binop_assign_trait!(ShlAssign, shl_assign, u64);
binop_assign_trait!(ShrAssign, shr_assign, i8);
binop_assign_trait!(ShrAssign, shr_assign, i16);
binop_assign_trait!(ShrAssign, shr_assign, i32);
binop_assign_trait!(ShrAssign, shr_assign, i64);
binop_assign_trait!(ShrAssign, shr_assign, u8);
binop_assign_trait!(ShrAssign, shr_assign, u16);
binop_assign_trait!(ShrAssign, shr_assign, u32);
binop_assign_trait!(ShrAssign, shr_assign, u64);


#[cfg(test)]
mod tests {
    use super::*;
    mod modular_arithmetic {
        use super::*;

        #[test]
        fn add_overflow() { assert_eq!(u15(MODULUS - 1) + u15(2), u15(1)); }

        #[test]
        fn mul_overflow() { assert_eq!(u15(MODULUS - 1) * u15(2), u15(MODULUS-2)); }

        #[test]
        #[should_panic]
        fn sub_underflow() { u15(0) - u15(2); }

        #[test]
        #[should_panic]
        fn div_by_zero() { u15(1) / u15(0); }

        #[test]
        fn min_val() { assert_eq!(u15::min_value(), u15(0)); }
        #[test]
        fn max_val() { assert_eq!(u15::max_value(), u15(MODULUS - 1)); }

        #[test]
        fn not_is_15b() { assert_eq!(!u15(0), u15(MODULUS - 1)); }
    }

    mod basics {
        use super::*;

        #[test]
        fn bitor() { assert_eq!(u15(16) | u15(17), u15(17)); }
        #[test]
        fn bitand() { assert_eq!(u15(16) & u15(17), u15(16)); }
        #[test]
        fn bitxor() { assert_eq!(u15(16) ^ u15(17), u15(1)); }


        #[test]
        fn bitor_assign() {
            let mut u = u15(16);
            u |= u15(17);
            assert_eq!(u, u15(17));
        }

        #[test]
        fn bitand_assign() {
            let mut u = u15(16);
            u &= u15(17);
            assert_eq!(u, u15(16));
        }

        #[test]
        fn bitxor_assign() {
            let mut u = u15(16);
            u ^= u15(17);
            assert_eq!(u, u15(1));
        }

        #[test]
        fn not() {
            assert_eq!(!u15(16), u15(32751));
        }

        #[test]
        fn add() { assert_eq!(u15(16) + u15(17), u15(33)); }
        #[test]
        fn mul() { assert_eq!(u15(16) * u15(10), u15(160)); }
        #[test]
        fn sub() { assert_eq!(u15(17) - u15(16), u15(1)); }
        #[test]
        #[should_panic]
        fn sub_panic() { u15(16) - u15(17); }
        #[test]
        fn div() { assert_eq!(u15(16) / u15(4), u15(4)); }
        #[test]
        fn div_non_even() { assert_eq!(u15(16) / u15(5), u15(3)); }
        #[test]
        fn rem() { assert_eq!(u15(16) % u15(4), u15(0)); }
        #[test]
        fn rem_non_even() { assert_eq!(u15(16) % u15(5), u15(1)); }

        #[test]
        fn add_assign() { let mut u = u15(16); u += u15(17); assert_eq!(u, u15(33)); }
        #[test]
        fn mul_assign() { let mut u = u15(16); u *= u15(10); assert_eq!(u, u15(160)); }
        #[test]
        fn sub_assign() { let mut u = u15(16); u -= u15(10); assert_eq!(u, u15(6)); }
        #[test]
        #[should_panic]
        fn sub_panic_assign() { let mut u = u15(16); u -= u15(18); }
        #[test]
        fn div_assign() { let mut u = u15(16); u /= u15(4); assert_eq!(u, u15(4)); }
        #[test]
        fn div_non_even_assign() { let mut u = u15(16); u /= u15(5); assert_eq!(u, u15(3)); }
        #[test]
        fn rem_assign() { let mut u = u15(16); u %= u15(4); assert_eq!(u, u15(0)); }
        #[test]
        fn rem_assign_nonzero_remainder() { let mut u = u15(16); u %= u15(5); assert_eq!(u, u15(1)); }

        #[test]
        fn shr() { assert_eq!(u15(8) << u15(1), u15(16)); }

        #[test]
        fn shr_i8() { assert_eq!(u15(8) << 1 as i8, u15(16)); }
        #[test]
        fn shr_i16() { assert_eq!(u15(8) << 1 as i16, u15(16)); }
        #[test]
        fn shr_i32() { assert_eq!(u15(8) << 1 as i32, u15(16)); }
        #[test]
        fn shr_i64() { assert_eq!(u15(8) << 1 as i64, u15(16)); }
        #[test]
        fn shr_u8() { assert_eq!(u15(8) << 1 as u8, u15(16)); }
        #[test]
        fn shr_u16() { assert_eq!(u15(8) << 1 as u16, u15(16)); }
        #[test]
        fn shr_u32() { assert_eq!(u15(8) << 1 as u32, u15(16)); }
        #[test]
        fn shr_u64() { assert_eq!(u15(8) << 1 as u64, u15(16)); }

        #[test]
        fn shl() { assert_eq!(u15(8) >> u15(1), u15(4)); }

        #[test]
        fn shl_i8() { assert_eq!(u15(8) >> 1 as i8, u15(4)); }
        #[test]
        fn shl_i16() { assert_eq!(u15(8) >> 1 as i16, u15(4)); }
        #[test]
        fn shl_i32() { assert_eq!(u15(8) >> 1 as i32, u15(4)); }
        #[test]
        fn shl_i64() { assert_eq!(u15(8) >> 1 as i64, u15(4)); }
        #[test]
        fn shl_u8() { assert_eq!(u15(8) >> 1 as u8, u15(4)); }
        #[test]
        fn shl_u16() { assert_eq!(u15(8) >> 1 as u16, u15(4)); }
        #[test]
        fn shl_u32() { assert_eq!(u15(8) >> 1 as u32, u15(4)); }
        #[test]
        fn shl_u64() { assert_eq!(u15(8) >> 1 as u64, u15(4)); }
    }
}

