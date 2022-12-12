pub trait Bitwise: private::Sealed {}

mod private {
    use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};

    pub trait Sealed:
        Sized
        + Copy
        + Not<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Shr<u32, Output = Self>
        + Shl<u32, Output = Self>
        + BitAnd<Output = Self>
        + Eq
        + PartialOrd
    {
        fn zero() -> Self;
        fn one() -> Self;

        fn update_bit_unchecked(self, index: usize, new_value: bool) -> Self;
        fn update_bit(self, index: usize, new_value: bool) -> Option<Self>;
        fn set_bit_unchecked(self, index: usize) -> Self;
        fn set_bit(self, index: usize) -> Option<Self>;
        fn clear_bit_unchecked(self, index: usize) -> Self;
        fn clear_bit(self, index: usize) -> Option<Self>;
        fn flip_bit_unchecked(self, index: usize) -> Self;
        fn flip_bit(self, index: usize) -> Option<Self>;
        fn flip_all(self) -> Self;
        fn clear(self) -> Self;
        // fn fill_range(self, left: usize, right: usize) -> Self;
        fn fill_all(self) -> Self;

        fn parity(self) -> usize;
        fn hamming_distance(self, other: Self) -> usize;
        fn bit_size() -> usize;

        fn at_unchecked(self, index: usize) -> bool;
        fn at(self, index: usize) -> Option<bool>;
    }
}

macro_rules! check_bit_index_or_return_none {
    ($bit:expr, $max_bits: expr) => {
        if $bit >= $max_bits {
            return None;
        }
    };
}

macro_rules! impl_bitwise {
    ($($max_bits:expr => $t:ident),*) => {$(
        impl Bitwise for $t {}

        impl private::Sealed for $t {

            fn zero() -> Self {
                0
            }
            fn one() -> Self {
                1
            }

            fn update_bit_unchecked(self, index: usize, new_value: bool) -> Self {
                if new_value {
                    self.set_bit_unchecked(index)
                } else {
                    self.clear_bit_unchecked(index)
                }
            }

            fn update_bit(self, index: usize, new_value: bool) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.update_bit_unchecked(index, new_value))
            }

            fn set_bit_unchecked(self, index: usize) -> Self {
                self | (Self::one() << index)
            }

            fn set_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.set_bit_unchecked(index))
            }

            fn clear_bit_unchecked(self, index: usize) -> Self {
                self & !(Self::one() << index)
            }

            fn clear_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.clear_bit_unchecked(index))
            }

            fn flip_bit_unchecked(self, index: usize) -> Self {
                self ^ (Self::one() << index)
            }

            fn flip_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.flip_bit_unchecked(index))
            }

            fn flip_all(self) -> Self {
                !self
            }

            fn clear(self) -> Self {
                self & 0
            }

            // TODO: test
            // fn fill_range(self, left: usize, right: usize) -> Self {
            //     let range = (((1 << (left - 1)) - 1) ^ ((1 << right) - 1));
            //     self | range
            // }

            fn fill_all(self) -> Self {
                self.clear().flip_all()
            }

            fn parity(self) -> usize {
                (self.count_ones() as usize) & (Self::one() as usize)
            }

            fn hamming_distance(self, other: Self) -> usize {
                (self ^ other).count_ones() as usize
            }

            fn bit_size() -> usize {
                $max_bits
            }

            fn at_unchecked(self, index: usize) -> bool {
                let mask = (Self::one() << index);
                (self & mask) == mask
            }

            fn at(self, index: usize) -> Option<bool> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.at_unchecked(index))
            }
        }
    )*};
}

const I8_BITS: usize = 8;
const I16_BITS: usize = 16;
const I32_BITS: usize = 32;
const I64_BITS: usize = 64;
const I128_BITS: usize = 128;

const U8_BITS: usize = 8;
const U16_BITS: usize = 16;
const U32_BITS: usize = 32;
const U64_BITS: usize = 64;
const U128_BITS: usize = 128;

#[cfg(feature = "i8")]
impl_bitwise!(I8_BITS => i8);

#[cfg(feature = "i16")]
impl_bitwise!(I16_BITS => i16);

#[cfg(feature = "i32")]
impl_bitwise!(I32_BITS => i32);

#[cfg(feature = "i64")]
impl_bitwise!(I64_BITS => i64);

#[cfg(feature = "i128")]
impl_bitwise!(I128_BITS => i128);

#[cfg(feature = "u8")]
impl_bitwise!(U8_BITS => u8);

#[cfg(feature = "u16")]
impl_bitwise!(U16_BITS => u16);

#[cfg(feature = "u32")]
impl_bitwise!(U32_BITS => u32);

#[cfg(feature = "u64")]
impl_bitwise!(U64_BITS => u64);

#[cfg(feature = "u128")]
impl_bitwise!(U128_BITS => u128);

#[cfg(test)]
mod tests {
    use crate::private::Sealed;

    #[test]
    fn set_bit() {
        let number: i8 = 0b00010;
        let expected: i8 = 0b10010;
        let other = number.update_bit(4, true).unwrap();
        assert_eq!(other, expected);
    }
}
