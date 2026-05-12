#[cfg(test)]
mod tests {
    use crate::rx;
    use crate::builder::build;

    #[test]
    fn test_valid_hex() {
        rx!("hi");
    }
}
