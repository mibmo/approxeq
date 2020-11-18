use super::*;

macro_rules! test {
    ($type1:ident, $type2:ident) => {{
        let a = 5 as $type1;
        let b = 3 as $type2;
        let c = 1 as $type2;
        assert!(a.aeq(&b));
        assert!(a.nae(&c));
    }};
}

macro_rules! gen_tests {
    ($type:ident) => {
        #[test]
        fn $type() {
            test!($type, u8);
            test!($type, u16);
            test!($type, u32);
            test!($type, u64);
            test!($type, u128);
            test!($type, usize);

            test!($type, i8);
            test!($type, i16);
            test!($type, i32);
            test!($type, i64);
            test!($type, i128);
            test!($type, isize);

            test!($type, f32);
            test!($type, f64);
        }
    };
}

gen_tests!(u8);
gen_tests!(u16);
gen_tests!(u32);
gen_tests!(u64);
gen_tests!(u128);
gen_tests!(usize);

gen_tests!(i8);
gen_tests!(i16);
gen_tests!(i32);
gen_tests!(i64);
gen_tests!(i128);
gen_tests!(isize);

gen_tests!(f32);
gen_tests!(f64);
