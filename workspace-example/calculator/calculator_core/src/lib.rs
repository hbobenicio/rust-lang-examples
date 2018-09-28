pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mult(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("You shall not divide by zero!");
    }

    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-5, 4), -1);
    }
}
