use std::iter::Peekable;
use std::str::Chars;

const METACHARACTERS: [char; 15] = ['.','*','+','?','^','$','{','}','[',']','(',')',']','|','\\'];

//TODO: keep in mind there are other cases
pub enum RegexTree {
    Literal(char), // "abc" — matches literally
    Wildcard, // . is a wildcard                            
    Class(Vec<ClassRange>, bool), // [a-f0-9] — matches any char in the class
    Anchor(AnchorKind), // ^ or $ or \b \B
    Shorthand(char), // \d \w \D \S are all shorthands (idk about this one) 
    Sequence(Vec<RegexTree>), // ab — two things in sequence
    Alternation(Vec<RegexTree>), // a|b — either this or that
    Repeat {
        node: Box<RegexTree>,
        min: usize,
        capturing: bool,
        max: Option<usize>, // None means unbounded
    }, // a+ a* a? a{3,6} — repetition
    Group {
        node: Box<RegexTree>,
        index: usize,
    }, // (abc) — captured group
} 

impl RegexTree{ 
    pub fn push(&mut self, node: RegexTree){
        match self{
            RegexTree::Sequence(ref mut nodes)=> nodes.push(node),
            _=>panic!("push called on non-sequence")
        }
    }
    pub fn push_range(&mut self, range: ClassRange){
        match self{
            RegexTree::Class(ref mut ranges, _)=>ranges.push(range),
            _=>panic!("push called on non-class")
        }
    }
}

pub enum AnchorKind {
    Start, // ^
    End,   // $
    WordBoundary, // \B
    NonWord, // \b

}
pub struct ClassRange {
    start: char,
    end: char,
}

fn parse_class(chars:&mut Peekable<Chars>)->RegexTree{
    let negation=*chars.peek().unwrap()=='^';
    if negation{ chars.next(); }
    let mut class=RegexTree::Class(Vec::new(),negation);
    while let Some(ch)=chars.next(){
        if ch==']'{ break; };
        match chars.peek() {
            Some(&'-') => {
                chars.next();
                match chars.next() {
                    Some(']') | None => {
                        class.push_range(ClassRange { start: ch, end: ch });
                        class.push_range(ClassRange { start: '-', end: '-' });
                        break;
                    }
                    Some(end) => {
                        //TODO: validate ch <= end
                        class.push_range(ClassRange { start: ch, end });
                    }
                }
            }
            _ => {
                class.push_range(ClassRange { start: ch, end: ch });
            }
        }
    }
    class
}

pub fn parse(x: &str){
    let mut tree=RegexTree::Sequence(Vec::new());
    let mut chars = x.chars().peekable();
    while let Some(ch)=chars.next(){
        //TODO:  replace with a match
        //TODO:  reminder that anchor needs \b and \B
        if METACHARACTERS.contains(&ch){ tree.push(RegexTree::Literal(ch)); } // 
        if ch=='.'{ tree.push(RegexTree::Wildcard); }
        if ch=='['{ chars.next(); tree.push(parse_class(&mut chars));
        if ch=='^'{ tree.push(RegexTree::Anchor(AnchorKind::Start)); }
        if ch=='$'{ tree.push(RegexTree::Anchor(AnchorKind::End)); }
                
        }
    }
}
