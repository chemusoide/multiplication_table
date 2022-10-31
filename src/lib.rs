#[cfg(test)]
mod tests {
    #[test]
    fn test_multiplication01() {
        let x = 2;
        let number = 5;
        let multiplication = x*number;
        assert_eq!(multiplication, 10);
    }

    #[test]
    fn test_multiplication02() {
        let x = 3;
        let number = 7;
        let multiplication = x*number;
        assert_eq!(multiplication, 21);
    }
}
