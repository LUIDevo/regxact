#[cfg(test)]
mod tests {
    use crate::rx;
    use crate::builder::RegxactBuilder;

    #[test]
    fn test_valid_hex() {
        rx!("hi");
    }
}
