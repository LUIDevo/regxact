use std::iter::Peekable;
use std::str::Chars;

const METACHARACTERS: [char; 15] = ['.','*','+','?','^','$','{','}','[',']','(',')',']','|','\\'];

//TODO: keep in mind there are other cases
#[derive(Clone)]
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
        capturing: bool,
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
    if s.contains(','){
        let parts:Vec<&str>=s.splitn(2, ',').collect();
        let min=parts[0].parse::<usize>().ok()?;
        let max=if parts[1].is_empty(){
            None
        } else {
            Some(parts[1].parse::<usize>().ok()?)
        };
        if let Some(max_val) = max { if min > max_val { return None; } }
        Some((min, max))
    } else{
        let n=s.parse::<usize>().ok()?;
        Some((n, Some(n)))
    }
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
    // let mut tree=RegexTree::Sequence(Vec::new());
    let mut chars = x.chars().peekable();
    let mut alternation=RegexTree::Alternation(Vec::new());
    let mut index: usize=0;
    let mut stack: Vec<Vec<RegexTree>>=vec![Vec::new()];
    while let Some(ch)=chars.next(){
        match ch {
            '.'=>stack.last_mut().unwrap().push(RegexTree::Wildcard),
            '['=>{
                chars.next(); 
                stack.last_mut().unwrap().push(parse_class(&mut chars));
            },
            '^'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::Start)),
            '$'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::End)),
            '\\'=>match chars.next(){
                Some('B')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::WordBoundary)),
                Some('b')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::NonWord)),
                Some(c)=>stack.last_mut().unwrap().push(RegexTree::Shorthand(c)),
                None=>panic!("trailing backslash"),
            }
            '*'=>{
                let prev=stack.last_mut().unwrap().pop().unwrap();
                stack.last_mut().unwrap().push(RegexTree::Repeat{node: Box::new(prev), min: 0, max: None});
            },
            '+'=>{
                let prev=stack.last_mut().unwrap().pop().unwrap();
                stack.last_mut().unwrap().push(RegexTree::Repeat{node: Box::new(prev), min: 1, max: None});
            },
            '?'=>{
                let prev=stack.last_mut().unwrap().pop().unwrap();
                stack.last_mut().unwrap().push(RegexTree::Repeat{node: Box::new(prev), min: 0, max: Some(1)});
            },
            '{'=>{
                let prev=stack.last_mut().unwrap().pop().unwrap();
                chars.next();
                index+=1;
                stack.last_mut().unwrap().push(parse_repeat(&mut x.chars().collect::<Vec<char>>(),&mut index, prev));
            },
            '('=>{
                stack.push(Vec::new());  
            },
            ')'=>{
                let prev=stack.pop().unwrap();
                stack[0].extend(prev);
            },
            c=>match chars.next(){
                Some('|')=>{
                    alternation.nodes_mut().extend(stack.last_mut().unwrap().clone());
                    stack.last_mut().unwrap().clear();
                }
                _=>stack.last_mut().unwrap().push(RegexTree::Literal(ch))
            }
        }
        index+=1;
    }
    if !alternation.is_empty(){
        alternation.nodes_mut().push(RegexTree::Sequence(stack.remove(0)));
        return alternation;
    }
    RegexTree::Sequence(stack.remove(0))
}
