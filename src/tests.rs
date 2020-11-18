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

macro_rules! int_tests {
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

macro_rules! float_tests {
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

// tests

int_tests!(u8);
int_tests!(u16);
int_tests!(u32);
int_tests!(u64);
int_tests!(u128);
int_tests!(usize);

int_tests!(i8);
int_tests!(i16);
int_tests!(i32);
int_tests!(i64);
int_tests!(i128);
int_tests!(isize);

float_tests!(f32);
float_tests!(f64);
