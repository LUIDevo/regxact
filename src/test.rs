#[cfg(test)]
mod tests {
    use crate::{rx};
    use crate::rx::Rx;
    use crate::regex_tree::{ClassRange, RegexTree};
    use crate::error::RegxactError;
    use crate::error::PerformanceError;
    use crate::error::CharacterClassError;
    use crate::allow::Allow;
    use crate::regex_tree::AnchorKind;
    use std::collections::HashSet;

    #[test]
    fn test_single_literal() {
        let tree=RegexTree::Sequence(vec!(RegexTree::Literal('a')));
        let result=Rx{pattern: "a".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!("a"), Result::Ok(result));
    }
    
    #[test]
    fn test_literal_sequence() {
        let tree=RegexTree::Sequence(vec![
            RegexTree::Literal('a'),
            RegexTree::Literal('b'),
            RegexTree::Literal('c'),
            RegexTree::Literal('d'),
            RegexTree::Literal('e'),
            RegexTree::Literal('f'),
        ]);
        let result=Rx{pattern: "abcdef".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!("abcdef"), Result::Ok(result));
    }

    #[test]
    fn test_alternation() {
        let tree=RegexTree::Alternation(vec![
            RegexTree::Literal('a'),
            RegexTree::Literal('b'),
        ]);
        let result=Rx{pattern: "a|b".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!("a|b"), Result::Ok(result));
    }

    #[test]
    fn test_error_nested_quantifier() {
        let result=RegxactError::Performance(PerformanceError::NestedQuantifier);
        assert_eq!(rx!("(a+)+"), Err(result));
    }

   fn test_error_unbounded_1() {
        let result=RegxactError::Performance(PerformanceError::NestedQuantifier);
        assert_eq!(rx!("(.*)*"), Err(result));
    }

   fn test_error_unbounded_2() {
        let result=RegxactError::Performance(PerformanceError::NestedQuantifier);
        assert_eq!(rx!(".*.*"), Err(result));
    }

    #[test]
    fn test_error_nested_quantifier_allows() {
        let mut allows=HashSet::new();
        allows.insert(Allow::Exponential);
        let tree=RegexTree::Sequence(
            vec![
            RegexTree::Repeat { node: Box::new(RegexTree::Group { node: Box::new(RegexTree::Sequence(vec!(RegexTree::Repeat { node: Box::new(RegexTree::Literal('a')), min: 1, max: None }))), index: 0, capturing: true }) , min: 1, max: None }
            ]
        );
        let result=Rx{pattern:"(a+)+".to_string(), tree, allows, contract: None};
        assert_eq!(rx!("(a+)+", allow="exponential"), Ok(result));
    }

    #[test]
    fn test_error_duplicate_alternation(){
        let result=RegxactError::Performance(PerformanceError::DuplicateAlternation);
        assert_eq!(rx!("(a|a)"), Err(result));
    }

    #[test]
    fn test_error_duplicate_alternation_three_branches() {
        let result=RegxactError::Performance(PerformanceError::DuplicateAlternation);
        assert_eq!(rx!("(a|b|a)"), Err(result));
    }

    #[test]
    fn test_error_duplicate_alternation_partial() {
        let result=RegxactError::Performance(PerformanceError::DuplicateAlternation);
        assert_eq!(rx!("(a|ab)"), Err(result));
    }

    #[test]
    fn test_shorthand() {
        assert_eq!(rx!(r"\s"), Err(RegxactError::CharacterClass(CharacterClassError::DotAll))); //BUG: TEMPORARY, THIS SHOULD ERROR OUT LATER DUE TO UNDECLARED CHARSET
    }

    #[test]
    fn test_repeat() {
        let tree=RegexTree::Sequence(vec![
            RegexTree::Repeat { node: Box::new(RegexTree::Literal('a')), min: 1, max: Some(2) }
        ]);
        let result=Rx{pattern: "a{1,2}".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!("a{1,2}"), Ok(result));
    }

    #[test]
    fn test_email() {
        let tree = RegexTree::Sequence(vec![
            RegexTree::Anchor(AnchorKind::Start),
            RegexTree::Repeat {
                node: Box::new(RegexTree::Class(vec![
                              ClassRange { start: 'a', end: 'z' },
                              ClassRange { start: 'A', end: 'Z' },
                              ClassRange { start: '0', end: '9' },
                              ClassRange { start: '.', end: '.' },
                              ClassRange { start: '_', end: '_' },
                              ClassRange { start: '%', end: '%' },
                              ClassRange { start: '+', end: '+' },
                              ClassRange { start: '-', end: '-' },
                ], false)),
                min: 1,
                max: None,
            },
            RegexTree::Literal('@'),
            RegexTree::Repeat {
                node: Box::new(RegexTree::Class(vec![
                              ClassRange { start: 'a', end: 'z' },
                              ClassRange { start: 'A', end: 'Z' },
                              ClassRange { start: '0', end: '9' },
                              ClassRange { start: '.', end: '.' },
                              ClassRange { start: '-', end: '-' },
                ], false)),
                min: 1,
                max: None,
            },
            RegexTree::Literal('.'),
            RegexTree::Repeat {
                node: Box::new(RegexTree::Class(vec![
                              ClassRange { start: 'a', end: 'z' },
                              ClassRange { start: 'A', end: 'Z' },
                ], false)),
                min: 2,
                max: None,
            },
            RegexTree::Anchor(AnchorKind::End),
            ]);
        let result = Rx { pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$".to_string(), tree, allows: HashSet::new(), contract: None };
        assert_eq!(Rx::email(), result);
    }

    #[test]
    fn test_repeat_2() {
        assert_eq!(rx!("\\.[a-zA-Z]{2,}"), Ok(Rx {
            pattern: "\\.[a-zA-Z]{2,}".to_string(),
            tree: RegexTree::Sequence(vec![
                RegexTree::Literal('.'),
                RegexTree::Repeat { node: Box::new(RegexTree::Class(vec![
                        ClassRange { start: 'a', end: 'z' },
                        ClassRange { start: 'A', end: 'Z' },
                ], false)), min: 2, max: None }
            ]),
            allows: HashSet::new(),
            contract: None
        }));
    }

    #[test]
    fn test_test() -> Result<(), RegxactError> {
        let r = rx!("^a$")?;
        assert_eq!(r.test("a")?, true);
        Ok(())
    }

    #[test]
    fn test_test_err(){
        let r = rx!("a").unwrap();
        assert!(r.test("a").is_err());
    }

    #[test]
    fn test_test_email() -> Result<(), RegxactError> {
        let r = Rx::email();
        assert_eq!(r.test("hi@gmail.com")?, true);
        Ok(())
    }

    #[test]
    fn test_test_versioning() -> Result<(), RegxactError> {
        let r = Rx::versioning();
        assert_eq!(r.test("1.1.1")?, true);
        Ok(())
    }

    #[test]
    fn test_test_versioning_err() -> Result<(), RegxactError> {
        let r = Rx::versioning();
        assert_eq!(r.test("1.1.a")?, false);
        Ok(())
    }

    #[test]
    fn test_test_ipv4 () -> Result<(), RegxactError> {
        let r = Rx::ipv4();
        assert_eq!(r.test("185.107.80.231")?, true);
        Ok(())
    }

    #[test]
    fn test_test_ipv6 () -> Result<(), RegxactError> {
        let r = Rx::ipv6();
        assert_eq!(r.test("2001:0db8:85a3:0000:0000:8a2e:0370:7334")?, true);
        Ok(())
    }

    #[test]
    fn test_test_ipv6_err () -> Result<(), RegxactError> {
        let r = Rx::ipv6();
        assert_eq!(r.test("abcdef")?, false);
        Ok(())
    }

    #[test]
    fn test_test_slug () -> Result<(), RegxactError> {
        let r = Rx::slug();
        assert_eq!(r.test("my-blog-post")?, true);
        Ok(())
    }

    #[test]
    fn test_test_slug_err () -> Result<(), RegxactError> {
        let r = Rx::slug();
        assert_eq!(r.test("ssdiajdsidsalt")?, false);
        Ok(())
    }
}
