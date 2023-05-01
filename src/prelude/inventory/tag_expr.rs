use crate::prelude::{Error, Result};
use bet::BeTree;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("&")]
    AndOp,

    #[token("|")]
    OrOp,

    #[token("!")]
    NotOp,

    #[regex(r"[^!\&\|\t\n\r\f\(\) ]+", |lex| lex.slice().parse())]
    Tag(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOp {
    And,
    Or,
    Not,
}

fn parse(input: &str) -> Result<BeTree<BoolOp, String>> {
    let mut expr = BeTree::new();
    let lex = Token::lexer(input);

    for tok in lex {
        match tok {
            Token::OpenParen => expr.open_par(),
            Token::CloseParen => expr.close_par(),
            Token::AndOp => expr.push_operator(BoolOp::And),
            Token::OrOp => expr.push_operator(BoolOp::Or),
            Token::NotOp => expr.push_operator(BoolOp::Not),
            Token::Tag(tag) => expr.push_atom(tag),
            _ => {
                return Err(Box::new(Error::InvalidToken(format!(
                    "Invalid token in tag expression: {:?}",
                    tok
                ))));
            }
        }
    }

    Ok(expr)
}

/// Evaluate a boolean tag expression against a list of tags.
pub fn eval_tag_expr(expr: &str, tags: Vec<String>) -> Result<bool> {
    let expr = parse(expr)?;
    let res = expr.eval_faillible(
        // evaluate leafs
        |tag| Ok(tags.contains(tag)),
        // evaluate operators
        |op, a, b| match (op, b) {
            (BoolOp::And, Some(b)) => Ok(a & b),
            (BoolOp::Or, Some(b)) => Ok(a | b),
            (BoolOp::Not, None) => Ok(!a),
            _ => Err("unexpected operation"),
        },
        // short-circuit
        |op, a| match (op, a) {
            (BoolOp::And, false) => true,
            (BoolOp::Or, true) => true,
            _ => false,
        },
    )?;

    match res {
        Some(val) => Ok(val),
        None => unreachable!("No boolean were returned"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_tag_expr_should_return_appropriate_values() {
        let tags = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

        assert!(eval_tag_expr("foo", tags.clone()).unwrap());
        assert!(eval_tag_expr("foo | biz", tags.clone()).unwrap());
        assert!(!eval_tag_expr("foo & biz", tags.clone()).unwrap());
        assert!(eval_tag_expr("foo & (bar | biz)", tags.clone()).unwrap());
        assert!(!eval_tag_expr("foo & !(bar | biz)", tags.clone()).unwrap());
    }
}
