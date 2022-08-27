use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    POpen,
    PClose,
    Lit(u32),
}

impl Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::POpen => 4,
            Token::PClose => 4,
            Token::Pow => 3,
            Token::Mul => 2,
            Token::Div => 2,
            Token::Add => 1,
            Token::Sub => 1,
            Token::Lit(_) => 0,
        }
    }

    fn associativity(&self) -> Associativity {
        match self {
            Token::Pow => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Add => f.write_str("+"),
            Token::Sub => f.write_str("-"),
            Token::Mul => f.write_str("*"),
            Token::Div => f.write_str("/"),
            Token::Pow => f.write_str("^"),
            Token::POpen => f.write_str("("),
            Token::PClose => f.write_str(")"),
            Token::Lit(n) => f.write_fmt(format_args!("{n}")),
        }
    }
}

pub fn to_postfix(infix: &str) -> String {
    let (mut output, op_stack) = infix
        .chars()
        .map(|c| match c {
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '^' => Token::Pow,
            '(' => Token::POpen,
            ')' => Token::PClose,
            _ => Token::Lit(c.to_digit(10).unwrap()),
        })
        .fold(
            (vec![], VecDeque::<Token>::new()),
            |(mut out, mut op_stack), t| {
                match t {
                    Token::Lit(_) => out.push(t),
                    Token::POpen => op_stack.push_front(t),
                    Token::PClose => {
                        while let Some(head) = op_stack.pop_front() {
                            match head {
                                Token::POpen => break,
                                _ => out.push(head),
                            }
                        }
                    }
                    op => {
                        while let Some(head) = op_stack.get(0) {
                            match head {
                                Token::POpen => break,
                                _ if head.precedence() > op.precedence() => {
                                    out.push(op_stack.pop_front().unwrap())
                                }
                                _ if head.precedence() == op.precedence()
                                    && op.associativity() == Associativity::Left =>
                                {
                                    out.push(op_stack.pop_front().unwrap())
                                }
                                _ => break,
                            }
                        }
                        op_stack.push_front(op);
                    }
                };
                (out, op_stack)
            },
        );

    output.extend(op_stack);
    output
        .into_iter()
        .map(|t| t.to_string())
        .collect::<String>()
}
#[cfg(test)]
mod tests {
    use super::to_postfix;

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(
            actual, expected,
            "\nYour answer (left) is not the correct answer (right)"
        )
    }

    #[test]
    fn fixed_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("1^2^3"), "123^^");
    }
}
