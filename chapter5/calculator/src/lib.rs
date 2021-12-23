pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else {
        x / y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(12, 2), 14);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(12, 2), 10);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(12, 2), 24);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(12, 2), 6);
    }
}
