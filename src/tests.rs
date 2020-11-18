use super::*;

macro_rules! int_test {
    ($type1:ident, $type2:ident) => {{
        let a: $type1 = 5;
        let b: $type2 = 3;
        let c: $type2 = 1;
        assert!(a.aeq(&b));
        assert!(a.nae(&c));
    }};
}

macro_rules! int_tests {
    ($type:ident) => {
        #[test]
        fn $type() {
            int_test!($type, u8);
            int_test!($type, u16);
            int_test!($type, u32);
            int_test!($type, u64);
            int_test!($type, u128);
            int_test!($type, usize);
            int_test!($type, i8);
            int_test!($type, i16);
            int_test!($type, i32);
            int_test!($type, i64);
            int_test!($type, i128);
            int_test!($type, isize);
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
