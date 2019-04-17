//! This example demonstrates how TryFrom<T> works.
//! 
//! It's analogous of the From<T> trait, but for convertions
//! that may fail.
//! 
//! It also autoimplements a "Into<T>" counterpart, the TryInto<T> trait,
//! just like the From<T> trait also autoimplements Into<T> for you.

use std::convert::TryFrom;

struct MaybeSomething {
    pub maybe_value: Option<i32>,
}

struct Something {
    pub value: i32,
}

impl TryFrom<MaybeSomething> for Something {
    type Error = &'static str;

    fn try_from(other: MaybeSomething) -> Result<Self, Self::Error> {
        let value = other.maybe_value.ok_or("there is no value!")?; 
        Ok(Something {value})
    }
}

fn main() {
    let maybe_something = MaybeSomething{
        maybe_value: Some(42),
    };

    match Something::try_from(maybe_something) {
        Ok(something) => println!("convertion is ok! got value: {}", something.value),
        Err(err) => eprintln!("convertion failed: {}", err),
    };

    let maybe_something = MaybeSomething {
        maybe_value: None,
    };

    match Something::try_from(maybe_something) {
        Ok(something) => println!("convertion is ok! got value: {}", something.value),
        Err(err) => eprintln!("convertion failed: {}", err),
    };
}
