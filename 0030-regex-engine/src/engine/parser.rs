use std::{
    error::Error,
    fmt::{self, Display},
    mem::take
};

#[derive(Debug)]
enum AST{
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPref(usize),
    NorightParen,
    Empty
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidEscape(pos, c) => write!(f, "ParserError:Invalid escape pos={pos}: char='{c}'"),
            ParserError::InvalidRightParen(pos) => write!(f, "ParserError: Invalid right parenthesis pos={pos}"),
            ParserError::NoPrev(pos) => write!(f, "ParserError: No previous expression: pos={pos}"),
            ParserError::NorightParen(pos) => write!(f, "ParserError: No right parenthesis "),
            ParserError::Empty => write!(f, "Empty expression"),
        }
    }
}

impl Error for ParserError {}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParserError> {
    match c {
        '\\' | '(' | ')' | '|' | '*' | '+' | '?' => Ok(AST::Char(c)),
        _ => {
           Err(ParserError::InvalidEscape(pos, c))
        }
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
}

fn parser_plus_star_question(seq: &mut Vec<AST>, ast_type: PSQ, pos: usize) -> Result<(AST, PSQ), ParserError> {
    if let Some(prev) = seq.pop(){
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParserError::NoPrev(pos))
    }
}

fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();        
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

pub fn parse(expr: &ast) -> Result<AST, ParserError> {
    enum ParseState{
        Char,
        Escape
    }

    let mut seq = Vec::new();
    let mut seq_or = Vec::new();
    let mut stack = Vec::new();
    let mut state = ParseState::Char;

    for(i, c) in expr.chars().enumerate(){
        match &state {
            ParseState::Char => {
                match c {
                    '+' => parser_plus_star_question(&mut seq, PSQ::Plus, i)?,
                    '*' => parser_plus_star_question(&mut seq, PSQ::Star, i)?,
                    '?' => parser_plus_star_question(&mut seq, PSQ::Question, i)?,
                    '(' => {
                        let prev = take(&mut seq);
                        let prev_or = take(&mut seq_or);
                        stack.push((prev, prev_or));
                    },
                    ')'=> {
                        if let Some((mut prev, prev_or)) = stack.pop(){
                            if !seq.is_empty() {
                                seq_or.push(AST::Seq(seq));
                            }
                            if let Some(ast) = fold_or(seq_or){
                                prev.push(ast);
                            }
                            seq = prev;
                            seq_or = prev_or;
                        } else {
                            return Err(ParserError::InvalidRightParen(i));
                        }
                    },
                    '|' => {
                        if !seq.is_empty() {
                            return Err(ParserError::NoPrev(i));
                        } else {
                            let prev = take(&mut seq);
                            seq_or.push(AST::Seq(prev));
                        }
                    },

                    '\\' => state = ParseState::Escape,
                    _ => seq.push(AST::Char(c)),
                    
                    
                }
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }
    if !stack.is_empty() {
        return Err(Box::new(ParserError::NorightParen));
    }
    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }
    if let Some(ast) = fold_or(seq_or){
        Ok(ast)
    } else {
        Err(Box::new(ParserError::Empty))
    }
 }