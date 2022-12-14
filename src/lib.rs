// Copyright 2022 Developers of the xbitwise project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Extend Bitwise Library for Rust
//!
//! A Rust library that extends the basic functionality of bitwise operations
//!
//! ## Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! xbitwise = "0.1"
//! ```
//!
//! *Version requirement: xbitwise supports rustc 1.31 and up.*
//!
//! ## Bug reports
//!
//! You can report any bugs [here](https://github.com/LuisMBaezCo/xbitwise/issues).
//!
//! # License
//!
//! xbitwise is distributed under the terms of both the MIT license.
//!
//! See [LICENSE-MIT](LICENSE-MIT)

use std::ops::Bound::*;
use std::ops::RangeBounds;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};

pub trait Bitwise:
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
    /// Gets the status of the bit in the index position
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `true` and `false`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other = 0b00010.get_bit_unchecked(1);
    /// assert_eq!(other, true);
    /// ```
    fn get_bit_unchecked(self, index: usize) -> bool;

    /// Gets the status of the bit in the `index` position
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `None`, `Some(true)` and `Some(false)`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let number: i8 = 0b00010;
    /// let other = number.get_bit(1);
    /// assert_eq!(other, Some(true));
    /// ```
    fn get_bit(self, index: usize) -> Option<bool>;

    /// Turn on the bit in the index position
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let number: i8 = 0b00010;
    /// let other = number.set_bit_unchecked(2);
    /// assert_eq!(other, 0b00110);
    /// ```
    fn set_bit_unchecked(self, index: usize) -> Self;

    /// Turn on the bit in the index position
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `None`, `Some(integer)`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let number: i8 = 0b00010;
    /// let other: Option<i8> = number.set_bit(2);
    /// assert_eq!(other, Some(0b00110));
    /// ```
    fn set_bit(self, index: usize) -> Option<Self>;

    /// Turns on all bits in the specified range and merges the bits that are already on
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** No
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let number: i32 = 0b100;
    ///
    /// let other = number.set_range_unchecked(..10);
    /// assert_eq!(other, 0b1111111111);
    ///
    /// let other = number.set_range_unchecked(0..10);
    /// assert_eq!(other, 0b1111111111);
    ///
    /// let other = number.set_range_unchecked(5..7);
    /// assert_eq!(other, 0b01100100);
    ///
    /// let other = number.set_range_unchecked(5..=7);
    /// assert_eq!(other, 0b11100100);
    /// ```
    fn set_range_unchecked<R: RangeBounds<Self>>(self, range: R) -> Self;

    /// Turns on all bits in the specified range and merges the bits that are already on
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `None`, `Some(integer)`
    ///
    /// **Stable:** No
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let number: i32 = 0b100;
    /// let other: Option<i32> = number.set_range(..10);
    /// assert_eq!(other, Some(0b1111111111));
    ///
    /// let other: Option<i32> = number.set_range(0..10);
    /// assert_eq!(other, Some(0b1111111111));
    ///
    /// let other: Option<i32> = number.set_range(5..7);
    /// assert_eq!(other, Some(0b01100100));
    ///
    /// let other: Option<i32> = number.set_range(5..=7);
    /// assert_eq!(other, Some(0b11100100));
    /// ```
    fn set_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self>;

    /// Turn on all the bits
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: u8 = 0b100.set_all();
    /// assert_eq!(other, 0b11111111);
    ///
    /// let other: u16 = 0b100.set_all();
    /// assert_eq!(other, 0b1111111111111111);
    ///
    /// let other: u32 = 0b100.set_all();
    /// assert_eq!(other, 0b11111111111111111111111111111111);
    /// ```
    fn set_all(self) -> Self;

    /// Update the bit at position `index` with the value `new_value`
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: i8 = 0b00010.update_bit_unchecked(4, true);
    /// assert_eq!(other, 0b10010);
    /// ```
    fn update_bit_unchecked(self, index: usize, new_value: bool) -> Self;

    /// Update the bit at position `index` with the value `new_value`
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: Option<i8> = 0b00010.update_bit(4, true);
    /// assert_eq!(other, Some(0b10010));
    /// ```
    fn update_bit(self, index: usize, new_value: bool) -> Option<Self>;

    /// Turn off the bit in the index position
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: i8 = 0b00010001.clear_bit_unchecked(4);
    /// assert_eq!(other, 0b00000001);
    /// ```
    fn clear_bit_unchecked(self, index: usize) -> Self;

    /// Turn off the bit in the index position
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: Option<i8> = 0b00010001.clear_bit(4);
    /// assert_eq!(other, Some(0b00000001));
    /// ```
    fn clear_bit(self, index: usize) -> Option<Self>;

    /// Turn off all bits
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: i8 = 0b00010001.clear();
    /// assert_eq!(other, 0b00000000);
    /// ```
    fn clear(self) -> Self;

    /// Flips the bit at the index position
    ///
    /// **Note:** This function does not check that the `index` is within the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: i8 = 0b00010001.flip_bit_unchecked(2);
    /// assert_eq!(other, 0b00010101);
    /// ```
    fn flip_bit_unchecked(self, index: usize) -> Self;

    /// Flips the bit at the index position
    ///
    /// **Note:** Returns `None` when the `index` is not in the allowed range.
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: Option<i8> = 0b00010001.flip_bit(2);
    /// assert_eq!(other, Some(0b00010101));
    /// ```
    fn flip_bit(self, index: usize) -> Option<Self>;

    /// Flips all the bits
    ///
    /// **Possible values:** `integer`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// #[allow(overflowing_literals)]
    /// let other: i8 =   0b10010001.flip();
    /// assert_eq!(other, 0b01101110);
    /// ```
    fn flip(self) -> Self;

    /// Returns the parity of x, i.e. the number of 1-bits in x modulo 2.
    ///
    /// **Possible values:** `true` or `false`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: bool = 0b00000001.parity();
    /// assert_eq!(other, true);
    ///
    /// let other: bool = 0b10000001.parity();
    /// assert_eq!(other, false);
    /// ```
    fn parity(self) -> bool;

    /// Returns a number with the hamming distance, which is the number of positions at which
    /// the corresponding bits are different.
    /// 
    /// **Possible values:** `true` or `false`
    ///
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let other: usize = 0b00000000.hamming_distance(0b11000001);
    /// assert_eq!(other, 3);
    /// ```
    fn hamming_distance(self, other: Self) -> usize;

    /// Return a number with all bits off (an integer of value zero).
    /// 
    /// **Possible values:** `0`
    /// 
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// assert_eq!(i8::zero(), 0i8);
    /// ```
    fn zero() -> Self;

    /// Returns a number with the least significant bit on (an integer with value one).
    ///
    /// **Possible values:** `1`
    /// 
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// assert_eq!(i8::one(), 1i8);
    /// ```
    fn one() -> Self;

    /// Returns the number of bits of the number
    ///
    /// **Possible values:** `integer`
    /// 
    /// **Stable:** Yes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xbitwise::Bitwise;
    ///
    /// let expected: usize = 8;
    /// assert_eq!(i8::bit_size(), 8usize);
    /// ```
    fn bit_size() -> usize;
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

        impl Bitwise for $t {

            fn get_bit_unchecked(self, index: usize) -> bool {
                let mask = (Self::one() << index);
                (self & mask) == mask
            }

            fn get_bit(self, index: usize) -> Option<bool> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.get_bit_unchecked(index))
            }

            fn set_bit_unchecked(self, index: usize) -> Self {
                self | (Self::one() << index)
            }

            fn set_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.set_bit_unchecked(index))
            }

            fn set_range_unchecked<R: RangeBounds<Self>>(self, range: R) -> Self {
                let left = match range.start_bound() {
                    Included(val) => (*val) as Self,
                    Excluded(val) => (*val) + 1 as Self,
                    _ => 0,
                };

                let right = match range.end_bound() {
                    Included(val) => (*val) as Self,
                    Excluded(val) => (*val) - 1 as Self,
                    _ => ($max_bits as Self) - 2,
                };

                let range = (((1 << left) - 1) ^ ((1 << right) - 1)) | (1 << right);
                self | range
            }

            fn set_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self> {
                let left = match range.start_bound() {
                    Included(val) => (*val) as usize,
                    Excluded(val) => ((*val) + 1) as usize,
                    _ => 0,
                };

                let right = match range.end_bound() {
                    Included(val) => (*val) as usize,
                    Excluded(val) => ((*val) - 1) as usize,
                    _ => ($max_bits as usize) - 2 as usize,
                };
                check_bit_index_or_return_none!(left, $max_bits);
                check_bit_index_or_return_none!(right, $max_bits);
                Some(self.set_range_unchecked(range))
            }

            fn set_all(self) -> Self {
                self.clear().flip()
            }

            fn update_bit_unchecked(self, index: usize, new_value: bool) -> Self {
                if new_value { self.set_bit_unchecked(index) } else { self.clear_bit_unchecked(index) }
            }

            fn update_bit(self, index: usize, new_value: bool) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.update_bit_unchecked(index, new_value))
            }

            fn clear_bit_unchecked(self, index: usize) -> Self {
                self & !(Self::one() << index)
            }

            fn clear_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.clear_bit_unchecked(index))
            }

            fn clear(self) -> Self {
                self & 0
            }

            fn flip_bit_unchecked(self, index: usize) -> Self {
                self ^ (Self::one() << index)
            }

            fn flip_bit(self, index: usize) -> Option<Self> {
                check_bit_index_or_return_none!(index, $max_bits);
                Some(self.flip_bit_unchecked(index))
            }

            fn flip(self) -> Self {
                !self
            }

            fn parity(self) -> bool {
                ((self.count_ones() as Self) & Self::one()) == Self::one()
            }

            fn hamming_distance(self, other: Self) -> usize {
                (self ^ other).count_ones() as usize
            }

            fn zero() -> Self { 0 }

            fn one() -> Self { 1 }

            fn bit_size() -> usize { $max_bits }
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
    use crate::Bitwise;

    #[test]
    fn get_bit_unchecked() {
        let number: i8 = 0b00010;
        let other = number.get_bit_unchecked(1);
        assert_eq!(other, true);
    }

    #[test]
    fn get_bit() {
        let number: i8 = 0b00010;
        let other = number.get_bit(1);
        assert_eq!(other, Some(true));
    }

    #[test]
    fn set_bit_unchecked() {
        let number: i8 = 0b00010;
        let other = number.set_bit_unchecked(2);
        assert_eq!(other, 0b00110);
    }

    #[test]
    fn set_bit() {
        let number: i8 = 0b00010;
        let other: Option<i8> = number.set_bit(2);
        assert_eq!(other, Some(0b00110));
    }

    #[test]
    fn set_range_unchecked() {
        let number: i32 = 0b100;
        let other: i32 = number.set_range_unchecked(..10);
        assert_eq!(other, 0b1111111111);
        let other: i32 = number.set_range_unchecked(0..10);
        assert_eq!(other, 0b1111111111);
        let other: i32 = number.set_range_unchecked(5..7);
        assert_eq!(other, 0b01100100);
        let other: i32 = number.set_range_unchecked(5..=7);
        assert_eq!(other, 0b11100100);
    }

    #[test]
    fn set_range() {
        let number: i32 = 0b100;
        let other: Option<i32> = number.set_range(..10);
        assert_eq!(other, Some(0b1111111111));
        let other: Option<i32> = number.set_range(0..10);
        assert_eq!(other, Some(0b1111111111));
        let other: Option<i32> = number.set_range(5..7);
        assert_eq!(other, Some(0b01100100));
        let other: Option<i32> = number.set_range(5..=7);
        assert_eq!(other, Some(0b11100100));
    }

    #[allow(overflowing_literals)]
    #[test]
    fn set() {
        let other: i8 = 0b100.set_all();
        assert_eq!(other, 0b11111111);

        let other: i16 = 0b100.set_all();
        assert_eq!(other, 0b1111111111111111);

        let other: i32 = 0b100.set_all();
        assert_eq!(other, 0b11111111111111111111111111111111);

        let other: i64 = 0b100.set_all();
        assert_eq!(
            other,
            0b1111111111111111111111111111111111111111111111111111111111111111
        );

        let other: i128 = 0b100.set_all();
        assert_eq!(
            other,
            0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
        );
    }

    #[test]
    fn update_bit_unchecked() {
        let other: i8 = 0b00010.update_bit_unchecked(4, true);
        assert_eq!(other, 0b10010);
    }

    #[test]
    fn update_bit() {
        let other: Option<i8> = 0b00010.update_bit(4, true);
        assert_eq!(other, Some(0b10010));
    }

    #[test]
    fn clear_bit_unchecked() {
        let other: i8 = 0b00010001.clear_bit_unchecked(4);
        assert_eq!(other, 0b00000001);
    }

    #[test]
    fn clear_bit() {
        let other: Option<i8> = 0b00010001.clear_bit(4);
        assert_eq!(other, Some(0b00000001));
    }

    #[test]
    fn clear() {
        let other: i8 = 0b00010001.clear();
        assert_eq!(other, 0b00000000);
    }

    #[test]
    fn flip_bit_unchecked() {
        let other: i8 = 0b00010001.flip_bit_unchecked(2);
        assert_eq!(other, 0b00010101);
    }

    #[test]
    fn flip_bit() {
        let other: Option<i8> = 0b00010001.flip_bit(2);
        assert_eq!(other, Some(0b00010101));
    }

    #[allow(overflowing_literals)]
    #[test]
    fn flip() {
        let other: i8 = 0b00010001.flip();
        assert_eq!(other, 0b11101110);
    }

    #[test]
    fn parity() {
        let other: bool = 0b00000001.parity();
        assert_eq!(other, true);

        let other: bool = 0b10100000.parity();
        assert_eq!(other, false);
    }

    #[test]
    fn hamming_distance() {
        let other: usize = 0b00000000.hamming_distance(0b11000001);
        assert_eq!(other, 3);
    }

    #[test]
    fn zero() {
        assert_eq!(i8::zero(), 0i8);
    }

    #[test]
    fn one() {
        assert_eq!(i8::one(), 1i8);
    }

    #[test]
    fn bit_size() {
        assert_eq!(i8::bit_size(), 8usize);
    }
}
