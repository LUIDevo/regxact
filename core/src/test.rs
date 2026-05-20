#[cfg(test)]
mod tests {
    use crate::{rx};
    use crate::pattern::Pattern;
    use crate::regex_tree::RegexTree;
    use crate::error::RegxactError;
    use crate::error::PerformanceError;
    use std::collections::HashSet;

    #[test]
    fn test_single_literal() {
        let tree=RegexTree::Sequence(vec!(RegexTree::Literal('a')));
        let result=Pattern{pattern: "a".to_string(), tree, allows: HashSet::new(), contract: None};
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
        let result=Pattern{pattern: "abcdef".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!("abcdef"), Result::Ok(result));
    }

    #[test]
    fn test_alternation() {
        let tree=RegexTree::Alternation(vec![
            RegexTree::Literal('a'),
            RegexTree::Literal('b'),
        ]);
        let result=Pattern{pattern: "a|b".to_string(), tree, allows: HashSet::new(), contract: None};
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
    fn test_error_unneeded_repeat() {
        let result=RegxactError::Performance(PerformanceError::UnneededRepeat);
        assert_eq!(rx!("a{1}"), Err(result));
    }

    #[test]
    fn test_shorthand() {
        // let result=RegxactError::Performance(PerformanceError::UnneededRepeat);
        let tree=RegexTree::Sequence(vec![
            RegexTree::Shorthand('d'),
        ]);
        let result=Pattern{pattern: r"\d".to_string(), tree, allows: HashSet::new(), contract: None};
        assert_eq!(rx!(r"\d"), Ok(result)); //BUG: TEMPORARY, THIS SHOULD ERROR OUT LATER DUE TO UNDECLARED CHARSET
    }
}
