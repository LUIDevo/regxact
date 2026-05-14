use std::iter::Peekable;
use std::str::Chars;

const METACHARACTERS: [char; 15] = ['.','*','+','?','^','$','{','}','[',']','(',')',']','|','\\'];

//TODO: keep in mind there are other cases
#[derive(Clone)]
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
        max: Option<usize>, // None means unbounded
    }, // a+ a* a? a{3,6} — repetition
    Group {
        node: Box<RegexTree>,
        index: usize,
        capturing: bool,
    }, // (abc) — captured group
} 

impl RegexTree{ 
    pub fn nodes_mut(&mut self) -> &mut Vec<RegexTree> {
        match self {
            RegexTree::Sequence(ref mut nodes) => nodes,
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

#[derive(Clone)]
pub enum AnchorKind {
    Start, // ^
    End,   // $
    WordBoundary, // \B
    NonWord, // \b
}

#[derive(Clone)]
pub struct ClassRange {
    start: char,
    end: char,
}

fn parse_repeat_contents(s: &String)->Option<(usize, Option<usize>)>{
}
fn parse_repeat(c: &mut Vec<char>, i: &mut usize, node: RegexTree)->RegexTree{
    if let Some(close)=c[*i..].iter().position(|&c| c=='}'){
        let content: String=c[*i..*i+close].iter().collect();
        *i+=close+1;
        if let Some((min,max))=parse_repeat_contents(&content){
            return RegexTree::Repeat{node: Box::new(node), min, max};
        }
    }
    RegexTree::Literal('{')
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

pub fn parse(x: &str)->RegexTree {
    let mut tree=RegexTree::Sequence(Vec::new());
    let mut chars = x.chars().peekable();
    let mut alternation=RegexTree::Alternation(Vec::new());
    let mut index: usize=0;
    while let Some(ch)=chars.next(){
        match ch {
            '.'=>tree.nodes_mut().push(RegexTree::Wildcard),
            '['=>{
                chars.next(); 
                tree.nodes_mut().push(parse_class(&mut chars));
            },
            '^'=>tree.nodes_mut().push(RegexTree::Anchor(AnchorKind::Start)),
            '$'=>tree.nodes_mut().push(RegexTree::Anchor(AnchorKind::End)),
            '\\'=>match chars.next(){
                Some('B')=>tree.nodes_mut().push(RegexTree::Anchor(AnchorKind::WordBoundary)),
                Some('b')=>tree.nodes_mut().push(RegexTree::Anchor(AnchorKind::NonWord)),
                Some(c)=>tree.nodes_mut().push(RegexTree::Shorthand(c)),
                None=>panic!("trailing backslash"),
            }
            '*'=>{
                let prev=tree.nodes_mut().pop().unwrap();
                tree.nodes_mut().push(RegexTree::Repeat{node: Box::new(prev), min: 0, max: None});
            },
            '+'=>{
                let prev=tree.nodes_mut().pop().unwrap();
                tree.nodes_mut().push(RegexTree::Repeat{node: Box::new(prev), min: 1, max: None});
            },
            '?'=>{
                let prev=tree.nodes_mut().pop().unwrap();
                tree.nodes_mut().push(RegexTree::Repeat{node: Box::new(prev), min: 0, max: Some(1)});
            },
            '{'=>{
                let prev=tree.nodes_mut().pop().unwrap();
                chars.next();
                index+=1;
                tree.nodes_mut().push(parse_repeat(&mut x.chars().collect::<Vec<char>>(),&mut index, prev));
            }
            c=>match chars.next(){
                Some('|')=>{
                    alternation.nodes_mut().extend(tree.nodes_mut().clone());
                    tree.nodes_mut().clear();
                }
                _=>tree.nodes_mut().push(RegexTree::Literal(ch))
            }
        }
        index+=1;
    }
    if !alternation.is_empty(){
        alternation.nodes_mut().push(tree);
        return alternation;
    }
    tree
}
