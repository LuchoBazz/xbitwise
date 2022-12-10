pub trait Bitwise: private::Sealed {}

mod private {
    pub trait Sealed: Copy {
        type Type;

        fn has_high_bit_unchecked(self, bit: usize) -> bool;
        fn has_high_bit(self, bit: usize) -> Option<bool>;
        fn has_low_bit_unchecked(self, bit: usize) -> bool;
        fn has_low_bit(self, bit: usize) -> Option<bool>;

        fn set_bit_unchecked(self, bit: usize) -> Self;
        fn set_bit(self, bit: usize) -> Option<Self::Type>;
        fn set_high_unchecked(self, bit: usize) -> Self;
        fn set_high(self, bit: usize) -> Option<Self::Type>;
        fn set_low_unchecked(self, bit: usize) -> Self;
        fn set_low(self, bit: usize) -> Option<Self::Type>;
    }
}

macro_rules! impl_bitwise {
    ($($max_bits:expr => $t:ident),* as $conv_fn:ident) => {$(
        impl Bitwise for $t {}

        impl private::Sealed for $t {

            type Type = $t;

            fn set_bit_unchecked(self, bit: usize) -> Self {
                if self.has_high_bit_unchecked(bit) {
                    self.set_low_unchecked(bit)
                } else {
                    self.set_high_unchecked(bit)
                }
            }

            fn set_bit(self, bit: usize) -> Option<Self::Type> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self.set_bit_unchecked(bit))
            }

            fn set_high_unchecked(self, bit: usize) -> Self {
                self | ((1 as Self::Type) << (bit as Self::Type))
            }

            fn set_high(self, bit: usize) -> Option<Self::Type> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self.set_high_unchecked(bit))
            }

            fn set_low_unchecked(self, bit: usize) -> Self {
                self & !((1 as Self) << (bit as Self))

            }

            fn set_low(self, bit: usize) -> Option<Self::Type> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self.set_low_unchecked(bit))
            }

            fn has_high_bit_unchecked(self, bit: usize) -> bool {
                self & ((1 as Self::Type) << (bit as Self::Type)) > 0
            }

            fn has_high_bit(self, bit: usize) -> Option<bool> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self.has_high_bit_unchecked(bit))
            }

            fn has_low_bit_unchecked(self, bit: usize) -> bool {
                (self & ((1 as Self::Type) << (bit as Self::Type))) == 0
            }

            fn has_low_bit(self, bit: usize) -> Option<bool> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self.has_low_bit_unchecked(bit))
            }
        }
    )*};
}

const I8_BITS: usize = 8;
const I16_BITS: usize = 16;
const I32_BITS: usize = 32;
const I64_BITS: usize = 64;
const I128_BITS: usize = 128;

impl_bitwise!(
    I8_BITS   => i8,
    I16_BITS  => i16,
    I32_BITS  => i32,
    I64_BITS  => i64,
    I128_BITS => i128 as i128
);

#[cfg(test)]
mod tests {
    use crate::private::Sealed;

    #[test]
    fn set_bit() {
        let number: i8 = 0b00010;
        let expected: i8 = 0b10010;
        let other = number.set_bit(4).unwrap();
        assert_eq!(other, expected);
    }
}
