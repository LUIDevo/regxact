#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RegexTree {
    Literal(char), // 'a' — matches literally
    Wildcard, // . is the one and only wildcard                            
    Class(Vec<ClassRange>, bool), // [a-f0-9] — matches any char in the class
    Anchor(AnchorKind), // ^ or $ or \b \B
    Shorthand(char), // \d \w \D \S are all shorthands (idk about this one) 
    Sequence(Vec<RegexTree>), // the entire regex as a sequence
    Alternation(Vec<RegexTree>), // a|b — either this or that
    Repeat {
        node: Box<RegexTree>,
        min: usize,
        max: Option<usize>, // None means unbounded
    }, // a+ a* a? a{3,6} — repetition
    Group {
        node: Box<RegexTree>,
        index: usize,
        capturing: bool, //TODO: ADD CAPTURING
    }, // (abc) — captured group
} 

impl RegexTree{ 
    pub fn nodes(&self) -> &Vec<RegexTree> {
        match self {
            RegexTree::Sequence(ref nodes)|RegexTree::Alternation(ref nodes) => nodes,
            _ => panic!("not a Sequence"),
        }
    }
    pub fn nodes_mut(&mut self) -> &mut Vec<RegexTree> {
        match self {
            RegexTree::Sequence(ref mut nodes)|RegexTree::Alternation(ref mut nodes) => nodes,
            _ => panic!("not a Sequence"),
        }
    }
    pub fn push(&mut self, node: RegexTree){
        match self {
            RegexTree::Sequence(ref mut nodes)|RegexTree::Alternation(ref mut nodes) => nodes.push(node),
            _ => panic!("not a Sequence"),
        }
    }
    pub fn push_range(&mut self, range: ClassRange){
        match self{
            RegexTree::Class(ref mut ranges, _)=>ranges.push(range),
            _=>panic!("push called on non-class")
        }
    }
    pub fn is_empty(&mut self)->bool{
        match self{
            RegexTree::Alternation(ref mut arr)|RegexTree::Sequence(ref mut arr)=>arr.is_empty(),
            _=>panic!("is_empty called on non alternation or sequence"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AnchorKind {
    LineStart, // ^
    LineEnd,   // $
    StringStart, // \A
    StringEnd,   // \Z
    StringEndAbsolute,   // \z
    WordBoundary, // \B
    NonWord, // \b
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClassRange {
    pub start: char,
    pub end: char,
}

