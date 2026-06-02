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

fn parse_repeat(chars: &mut Vec<char>, i: &mut usize, node: RegexTree)->RegexTree{
    println!("{}", i);
    if let Some(close)=chars[*i..].iter().position(|&c| c=='}'){
        let content: String=chars[*i..*i+close].iter().collect();
        println!("{:?}", content);
        *i+=close;
        if let Some((min,max))=parse_repeat_contents(&content){
            return RegexTree::Repeat{node: Box::new(node), min, max};
        }
    }
    RegexTree::Literal('{')
}

fn parse_class(chars: &Vec<char>, i: &mut usize)->RegexTree{
    *i+=1;
    let negation = chars.get(*i) == Some(&'^');
    if negation{ *i+=1; }
    let mut class=RegexTree::Class(Vec::new(),negation);
    while *i < chars.len() && chars[*i] != ']' {
        let start=chars[*i];
        if chars.get(*i+1)==Some(&'-') && chars.get(*i+2).map_or(false, |&c| c != ']') {
            let end=chars[*i+2];
            //TODO: validate start <= end
            class.push_range(ClassRange { start, end });
            *i+=3;
        } else {
            class.push_range(ClassRange { start, end: start });
            *i+=1;
        }
    }
    class
}

fn group_prefix_end(chars: &Vec<char>, open: usize) -> usize {
    if chars.get(open + 1) != Some(&'?') {
        return open;
    }
    let find = |from: usize, close: char, default: usize| {
        chars[from..].iter().position(|&c| c == close).map_or(default, |p| from + p)
    };
    match chars.get(open + 2) {
        Some(':') | Some('=') | Some('!') => open + 2,
        Some('<') => match chars.get(open + 3) {
            Some('=') | Some('!') => open + 3,
            _ => find(open + 3, '>', open + 2),
        },
        Some('P') => find(open + 3, '>', open + 2),
        Some('\'') => find(open + 3, '\'', open + 2),
        _ => open + 1,
    }
}

pub fn parse(x: &str)->RegexTree {
    let mut chars = x.chars().collect::<Vec<char>>();
    let mut alternation=vec![false];
    let mut index: usize=0;
    let mut stack=vec![Vec::new()];
    while index < chars.len(){
        let ch=chars[index];
        // println!("main: {} {}", ch, list[index]); //TODO : REMOVE
        match ch {
            '.'=>stack.last_mut().unwrap().push(RegexTree::Wildcard),
            '['=>{
                let parsed_class=parse_class(&mut chars, &mut index);
                stack.last_mut().unwrap().push(parsed_class);
            },
            '^'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::LineStart)),
            '$'=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::LineEnd)),
            '\\'=>{
                match chars.get(index+1){
                    Some('b')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::WordBoundary)),
                    Some('B')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::NonWord)),
                    Some('A')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::StringStart)),
                    Some('Z')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::StringEnd)),
                    Some('z')=>stack.last_mut().unwrap().push(RegexTree::Anchor(AnchorKind::StringEndAbsolute)),
                    Some('.')=>stack.last_mut().unwrap().push(RegexTree::Literal('.')),
                    Some(c)=>stack.last_mut().unwrap().push(RegexTree::Shorthand(*c)),
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
                stack.last_mut().unwrap().push(parse_repeat(&mut chars, &mut index, prev));
            }
            '}'=>(),
            '('=>{
                stack.push(Vec::new());
                alternation.push(false);
                index = group_prefix_end(&chars, index); 
            },
            ')'=>{
                let prev=stack.pop().unwrap();
                let is_alt=alternation.pop().unwrap();
                let node = if is_alt {Box::new(RegexTree::Alternation(prev))} else {Box::new(RegexTree::Sequence(prev))};
                stack.last_mut().unwrap().push(RegexTree::Group{node: node, index: 0, capturing: true});//BUG: FIX INDEX
            },
            '|' => {
                *alternation.last_mut().unwrap()=true;
            },
            c=>{
                stack.last_mut().unwrap().push(RegexTree::Literal(c));
            },
        }
        index+=1;
    }
    if *alternation.last_mut().unwrap()==true{
        return RegexTree::Alternation(stack.remove(0));
    }
    RegexTree::Sequence(stack.remove(0))
}
