pub trait Bitwise: private::Sealed {}

mod private {
    pub trait Sealed: Copy {
        type Type;
        fn set_bit_unchecked(self, bit: usize) -> Self::Type;
        fn set_bit(self, bit: usize) -> Option<Self::Type>;
    }
}

// Adaptation of the original implementation at
// https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L188-L266
macro_rules! impl_bitwise {
    ($($max_bits:expr => $t:ident),* as $conv_fn:ident) => {$(
        impl Bitwise for $t {}

        impl private::Sealed for $t {

            type Type = $t;

            fn set_bit_unchecked(self, bit: usize) -> Self::Type {
                self | ( (1 as Self::Type) << (bit as Self::Type))
            }

            fn set_bit(self, bit: usize) -> Option<Self::Type> {
                if bit >= $max_bits {
                    return None;
                }
                Some(self | ( (1 as Self::Type) << (bit as Self::Type)))
            }
        }
    )*};
}

const I8_BITS: usize   = 8;
const I16_BITS: usize  = 16;
const I32_BITS: usize  = 32;
const I64_BITS: usize  = 64;
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
        let number  : i8 = 0b00010;
        let expected: i8 = 0b10010;
        let other = number
        .set_bit(4)
        .unwrap();
        assert_eq!(other, expected);
    }

}