use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

#[derive(Debug, PartialEq)]
enum Ast {
    UnOp(String, usize),
    BinOp(String, Box<Self>, Box<Self>),
}

impl Ast {
    fn imm(n: usize) -> Self {
        Self::UnOp("imm".to_string(), n)
    }

    fn arg(idx: usize) -> Self {
        Self::UnOp("arg".to_string(), idx)
    }

    fn add(lhs: Self, rhs: Self) -> Self {
        Self::BinOp("+".to_string(), Box::new(lhs), Box::new(rhs))
    }

    fn sub(lhs: Self, rhs: Self) -> Self {
        Self::BinOp("-".to_string(), Box::new(lhs), Box::new(rhs))
    }

    fn mul(lhs: Self, rhs: Self) -> Self {
        Self::BinOp("*".to_string(), Box::new(lhs), Box::new(rhs))
    }

    fn div(lhs: Self, rhs: Self) -> Self {
        Self::BinOp("/".to_string(), Box::new(lhs), Box::new(rhs))
    }
}

type TokenStream = Peekable<IntoIter<String>>;

trait Nom<T> {
    fn nom(&mut self) -> T;
}

impl Nom<String> for TokenStream {
    fn nom(&mut self) -> String {
        self.next().unwrap()
    }
}

struct Parser {
    tokens: TokenStream,
    args: HashMap<String, usize>,
}

impl Parser {
    fn new(tokens: TokenStream) -> Self {
        Self {
            tokens,
            args: HashMap::new(),
        }
    }

    // Grammar
    // -------
    // function   ::= '[' arg-list ']' expression
    //
    // arg-list   ::= /* nothing */
    //              | variable arg-list
    //
    // expression ::= term
    //              | expression '+' term
    //              | expression '-' term
    //
    // term       ::= factor
    //              | term '*' factor
    //              | term '/' factor
    //
    // factor     ::= number
    //              | variable
    //              | '(' expression ')'
    fn parse(&mut self) -> Ast {
        self.args();
        self.expression()
    }

    fn expression(&mut self) -> Ast {
        let mut lhs = self.term();

        while let Some(token) = self.tokens.peek() {
            match token.as_str() {
                "+" | "-" => {
                    let op = self.tokens.nom();
                    let rhs = self.term();
                    lhs = Ast::BinOp(op, Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }

        lhs
    }

    fn term(&mut self) -> Ast {
        let mut lhs = self.factor();

        while let Some(token) = self.tokens.peek() {
            match token.as_str() {
                "*" | "/" => {
                    let op = self.tokens.nom();
                    let rhs = self.factor();
                    lhs = Ast::BinOp(op, Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }

        lhs
    }

    fn factor(&mut self) -> Ast {
        let mut bytes = self.tokens.peek().unwrap().bytes();

        match bytes.next().unwrap() {
            // number
            b'0'..=b'9' => Ast::imm(self.tokens.nom().parse::<usize>().unwrap()),
            // expression
            b'(' => {
                self.tokens.nom(); // opening paren
                let e = self.expression();
                self.tokens.nom(); // closing paren
                e
            }
            // variable
            _ => {
                let var = self.tokens.nom();
                let idx = self.args.get(&var).unwrap();
                Ast::arg(*idx)
            }
        }
    }

    fn args(&mut self) {
        let mut pos = 0;

        loop {
            let next = self.tokens.nom();

            match next.as_str() {
                "[" => continue,
                "]" => break,
                _ => {
                    self.args.insert(next, pos);
                    pos += 1;
                }
            }
        }
    }
}

struct Compiler {
    // your code
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }

    fn tokenize(&self, program: &str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let mut iter = program.chars().peekable();

        while let Some(&c) = iter.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(tmp);
                }
                '0'..='9' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(tmp);
                }
                ' ' => {
                    iter.next();
                }
                _ => {
                    tokens.push(iter.next().unwrap().to_string());
                }
            }
        }

        tokens
    }

    fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn pass1(&mut self, program: &str) -> Ast {
        let tokens = self.tokenize(program);
        let iter = tokens.into_iter().peekable();
        Parser::new(iter).parse()
    }

    fn pass2(&mut self, ast: &Ast) -> Ast {
        todo!()
        // your code
    }

    fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        todo!()
        // your code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass1_binary() {
        let input = "[ first second ] (first + second) / 2";

        let mut c = Compiler::new();
        let ast = c.pass1(input);

        assert_eq!(
            ast,
            Ast::div(Ast::add(Ast::arg(0), Ast::arg(1)), Ast::imm(2))
        );
    }

    #[test]
    fn test_pass1_nary() {
        let input = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";

        let mut c = Compiler::new();
        let ast = c.pass1(input);

        assert_eq!(
            ast,
            Ast::div(
                Ast::sub(
                    Ast::add(
                        Ast::mul(Ast::mul(Ast::imm(2), Ast::imm(3)), Ast::arg(0)),
                        Ast::mul(Ast::imm(5), Ast::arg(1))
                    ),
                    Ast::mul(Ast::imm(3), Ast::arg(2))
                ),
                Ast::add(
                    Ast::add(Ast::imm(1), Ast::imm(3)),
                    Ast::mul(Ast::imm(2), Ast::imm(2))
                )
            )
        );
    }

    #[test]
    fn simulator() {
        assert_eq!(simulate(vec!["IM 7".to_string()], vec![3]), 7);
        assert_eq!(simulate(vec!["AR 1".to_string()], vec![1, 2, 3]), 2);
    }

    fn simulate(assembly: Vec<String>, argv: Vec<i32>) -> i32 {
        let mut r = (0, 0);
        let mut stack: Vec<i32> = vec![];

        for ins in assembly {
            let mut ws = ins.split_whitespace();
            match ws.next() {
                Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
                Some("AR") => {
                    r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize]
                }
                Some("SW") => r = (r.1, r.0),
                Some("PU") => stack.push(r.0),
                Some("PO") => r.0 = stack.pop().unwrap(),
                Some("AD") => r.0 += r.1,
                Some("SU") => r.0 -= r.1,
                Some("MU") => r.0 *= r.1,
                Some("DI") => r.0 /= r.1,
                _ => panic!("Invalid instruction encountered"),
            }
        }
        r.0
    }
}
