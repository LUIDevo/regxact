use std::iter::Peekable;
use std::str::Chars;
use crate::regex_tree::AnchorKind;
use crate::regex_tree::RegexTree;
use crate::regex_tree::ClassRange;

//TODO: keep in mind there are other cases
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

fn parse_repeat(c: &mut Vec<char>, i: &mut usize, node: RegexTree, chars:&mut Peekable<Chars>)->RegexTree{
    println!("{}", i);
    if let Some(close)=c[*i..].iter().position(|&c| c=='}'){
        let content: String=c[*i..*i+close].iter().collect();
        println!("{:?}", content);
        for _ in 0..=close { chars.next(); }
        *i+=close+1;
        if let Some((min,max))=parse_repeat_contents(&content){
            return RegexTree::Repeat{node: Box::new(node), min, max};
        }
    }
    RegexTree::Literal('{')
}

fn parse_class(chars:&mut Peekable<Chars>)->(RegexTree, usize){
    let mut chars_consumed: usize=3;
    let negation=*chars.peek().unwrap()=='^'; //BUG: THIS is probably bugged logic due to the previous line, fix later and implement tests for negation
    if negation{ chars.next(); }
    let mut class=RegexTree::Class(Vec::new(),negation);
    while let Some(ch)=chars.next(){
        chars_consumed+=1;
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
    (class,chars_consumed)
}

pub fn parse(x: &str)->RegexTree {
    let mut chars = x.chars().peekable();
    let mut list = x.chars().collect::<Vec<char>>();
    let mut alternation=vec![false];
    let mut index: usize=0;
    let mut stack=vec![Vec::new()];
    while let Some(ch)=chars.next(){
        println!("{} {}", ch, list[index]); //TODO : REMOVE
        match ch {
            '.'=>stack.last_mut().unwrap().push(RegexTree::Wildcard),
            '['=>{
                chars.next(); 
                let parsed_class=parse_class(&mut chars);
                stack.last_mut().unwrap().push(parsed_class.0);
                index+=parsed_class.1;
            },
            '^'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::Start)),
            '$'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::End)),
            '\\'=>{
                match chars.next(){
                Some('B')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::WordBoundary)),
                Some('b')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::NonWord)),
                Some('.')=>stack.last_mut().unwrap().push(RegexTree::Literal('.')),
                Some(c)=>stack.last_mut().unwrap().push(RegexTree::Shorthand(c)),
                None=>panic!("trailing backslash"),
                };
                index += 1;
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
            '{' => {
                let prev = stack.last_mut().unwrap().pop().unwrap();
                index += 1;
                stack.last_mut().unwrap().push(parse_repeat(&mut list, &mut index, prev, &mut chars));
            }
            '}'=>(),
            '('=>{
                stack.push(Vec::new());  
                alternation.push(false);
            },
            ')'=>{
                let prev=stack.pop().unwrap();
                let is_alt=alternation.pop().unwrap();
                let node = if is_alt {Box::new(RegexTree::Alternation(prev))} else {Box::new(RegexTree::Sequence(prev))};
                stack.last_mut().unwrap().push(RegexTree::Group{node: node, index: 0, capturing: true});//BUG: FIX INDEX
            },
            '|' => {
                *alternation.last_mut().unwrap()=true;
            }
            c=>{
                stack.last_mut().unwrap().push(RegexTree::Literal(c));
            }
        }
        index+=1;
    }
    if *alternation.last_mut().unwrap()==true{
        return RegexTree::Alternation(stack.remove(0));
    }
    RegexTree::Sequence(stack.remove(0))
}
