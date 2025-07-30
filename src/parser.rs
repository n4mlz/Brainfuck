use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Node {
    IncPtr,
    DecPtr,
    IncCell,
    DecCell,
    Output,
    Input,
    Loop(Vec<Node>),
}

fn parse_inner(iter: &mut dyn Iterator<Item = &Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(token) = iter.next() {
        match token {
            Token::IncPtr => nodes.push(Node::IncPtr),
            Token::DecPtr => nodes.push(Node::DecPtr),
            Token::IncCell => nodes.push(Node::IncCell),
            Token::DecCell => nodes.push(Node::DecCell),
            Token::Output => nodes.push(Node::Output),
            Token::Input => nodes.push(Node::Input),
            Token::LoopStart => {
                let body = parse_inner(iter);
                nodes.push(Node::Loop(body));
            }
            Token::LoopEnd => {
                break;
            }
        }
    }
    nodes
}

pub fn parse(tokens: &[Token]) -> Vec<Node> {
    let mut iter = tokens.iter();
    parse_inner(&mut iter)
}
