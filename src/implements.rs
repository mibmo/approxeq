use super::ApproxEq;

macro_rules! approxeq_impl_abs {
    ($type1:ident, $type2:ident) => {
        impl ApproxEq<$type2> for $type1 {
            fn aeq(&self, other: &$type2) -> bool {
                (self - *other as $type1).abs() <= 2 as _
            }
        }
    };
}

macro_rules! approxeq_impl_no_abs {
    ($type1:ident, $type2:ident) => {
        impl ApproxEq<$type2> for $type1 {
            fn aeq(&self, other: &$type2) -> bool {
                (self - *other as $type1) <= 2
            }
        }
    };
}

macro_rules! approxeq_impl_ints {
    ($type:ident) => {
        approxeq_impl_abs!($type, i8);
        approxeq_impl_abs!($type, i16);
        approxeq_impl_abs!($type, i32);
        approxeq_impl_abs!($type, i64);
        approxeq_impl_abs!($type, i128);
        approxeq_impl_abs!($type, isize);

        approxeq_impl_abs!($type, u8);
        approxeq_impl_abs!($type, u16);
        approxeq_impl_abs!($type, u32);
        approxeq_impl_abs!($type, u64);
        approxeq_impl_abs!($type, u128);
        approxeq_impl_abs!($type, usize);

        approxeq_impl_abs!($type, f32);
        approxeq_impl_abs!($type, f64);
    }
}

macro_rules! approxeq_impl_uints {
    ($type:ident) => {
        approxeq_impl_no_abs!($type, i8);
        approxeq_impl_no_abs!($type, i16);
        approxeq_impl_no_abs!($type, i32);
        approxeq_impl_no_abs!($type, i64);
        approxeq_impl_no_abs!($type, i128);
        approxeq_impl_no_abs!($type, isize);

        approxeq_impl_no_abs!($type, u8);
        approxeq_impl_no_abs!($type, u16);
        approxeq_impl_no_abs!($type, u32);
        approxeq_impl_no_abs!($type, u64);
        approxeq_impl_no_abs!($type, u128);
        approxeq_impl_no_abs!($type, usize);

        approxeq_impl_no_abs!($type, f32);
        approxeq_impl_no_abs!($type, f64);
    }
}

macro_rules! approxeq_impl_floats {
    ($type:ident) => {
        approxeq_impl_abs!(f32, $type);
        approxeq_impl_abs!(f64, $type);
    }
}

approxeq_impl_ints!(i8);
approxeq_impl_ints!(i16);
approxeq_impl_ints!(i32);
approxeq_impl_ints!(i64);
approxeq_impl_ints!(i128);
approxeq_impl_ints!(isize);
approxeq_impl_uints!(u8);
approxeq_impl_uints!(u16);
approxeq_impl_uints!(u32);
approxeq_impl_uints!(u64);
approxeq_impl_uints!(u128);
approxeq_impl_uints!(usize);

approxeq_impl_floats!(i8);
approxeq_impl_floats!(i16);
approxeq_impl_floats!(i32);
approxeq_impl_floats!(i64);
approxeq_impl_floats!(i128);
approxeq_impl_floats!(isize);

approxeq_impl_floats!(u8);
approxeq_impl_floats!(u16);
approxeq_impl_floats!(u32);
approxeq_impl_floats!(u64);
approxeq_impl_floats!(u128);
approxeq_impl_floats!(usize);

approxeq_impl_floats!(f32);
approxeq_impl_floats!(f64);
