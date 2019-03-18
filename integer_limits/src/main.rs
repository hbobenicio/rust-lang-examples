macro_rules! print_limits {
    ($t:ident) => {
        println!("{}: [{}, {}]", stringify!($t), std::$t::MIN, std::$t::MAX);
    };
}

macro_rules! print_all_limits {
    ( $( $t:ident ),* ) => {
        $(
            print_limits!($t);
        )*
    };
}

fn main() {
    print_all_limits!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
