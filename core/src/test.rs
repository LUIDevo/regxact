#[cfg(test)]
mod tests {
    use crate::{rx};
    use crate::rx::Rx;
    use crate::regex_tree::{ClassRange, RegexTree};
    use crate::error::RegxactError;
    use crate::error::PerformanceError;
    use crate::error::CharacterClassError;
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
        assert_eq!(Rx::email(), Ok(result));
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
}
