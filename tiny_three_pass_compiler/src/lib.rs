#![allow(dead_code)]
use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

#[derive(Debug, PartialEq)]
enum Ast {
    UnOp(String, usize),
    BinOp(String, Box<Self>, Box<Self>),
}

impl Ast {
    // Simplifies the AST by applying constant folding,
    // i.e., evaluating binary expression where both
    // inputs are immediate values. Folding is applied
    // bottom-up, requiring only one pass over the AST.
    fn fold(&self) -> Ast {
        match self {
            Self::BinOp(op, lhs, rhs) => {
                let lhs = lhs.fold();
                let rhs = rhs.fold();

                match (&lhs, &rhs) {
                    (Self::UnOp(cmd_lhs, n_lhs), Self::UnOp(cmd_rhs, n_rhs))
                        if cmd_lhs == "imm" && cmd_rhs == "imm" =>
                    {
                        let n = match op.as_str() {
                            "+" => n_lhs + n_rhs,
                            "-" => n_lhs - n_rhs,
                            "*" => n_lhs * n_rhs,
                            "/" => n_lhs / n_rhs,
                            _ => unreachable!(),
                        };

                        Self::UnOp("imm".to_string(), n)
                    }
                    _ => Self::BinOp(op.clone(), Box::new(lhs), Box::new(rhs)),
                }
            }
            Self::UnOp(op, n) => Self::UnOp(op.clone(), *n),
        }
    }

    // Transforms the AST into the following assembly language
    //
    // "IM n"     // load the constant value n into R0
    // "AR n"     // load the n-th input argument into R0
    // "SW"       // swap R0 and R1
    // "PU"       // push R0 onto the stack
    // "PO"       // pop the top value off of the stack into R0
    // "AD"       // add R1 to R0 and put the result in R0
    // "SU"       // subtract R1 from R0 and put the result in R0
    // "MU"       // multiply R0 by R1 and put the result in R0
    // "DI"       // divide R0 by R1 and put the result in R0
    fn transform(&self, asm: &mut Vec<String>) {
        match self {
            Self::BinOp(op, lhs, rhs) => {
                match (&**lhs, &**rhs) {
                    (Ast::UnOp(_, _), _) => {
                        rhs.transform(asm);
                        asm.push("SW".to_string());
                        lhs.transform(asm);
                    }
                    (Ast::BinOp(_, _, _), Ast::UnOp(_, _)) => {
                        lhs.transform(asm);
                        asm.push("SW".to_string());
                        rhs.transform(asm);
                    }
                    (Ast::BinOp(_, _, _), Ast::BinOp(_, _, _)) => {
                        lhs.transform(asm);
                        asm.push("PU".to_string());
                        rhs.transform(asm);
                        asm.push("SW".to_string());
                        asm.push("PO".to_string());
                    }
                }

                let op = match op.as_str() {
                    "*" => "MU".to_string(),
                    "/" => "DI".to_string(),
                    "+" => "AD".to_string(),
                    "-" => "SU".to_string(),
                    _ => unreachable!(),
                };

                asm.push(op);
            }
            Self::UnOp(op, n) => {
                if op == "imm" {
                    asm.push(format!("IM {}", n));
                } else {
                    asm.push(format!("AR {}", n));
                }
            }
        }
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
            b'0'..=b'9' => Ast::UnOp("imm".to_string(), self.tokens.nom().parse().unwrap()),
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
                Ast::UnOp("arg".to_string(), *idx)
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

struct Compiler;

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
        ast.fold()
    }

    fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        let mut asm = vec![];
        ast.transform(&mut asm);
        asm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions for building ASTs in tests
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

    #[test]
    fn test_pass1_1() {
        let input = "[ first second ] (first + second) / 2";

        let mut c = Compiler::new();
        let ast = c.pass1(input);

        assert_eq!(
            ast,
            Ast::div(Ast::add(Ast::arg(0), Ast::arg(1)), Ast::imm(2))
        );
    }

    #[test]
    fn test_pass1_2() {
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
    fn test_pass2() {
        let input = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";

        let mut c = Compiler::new();
        let ast = c.pass1(input);
        let ast = c.pass2(&ast);

        assert_eq!(
            ast,
            Ast::div(
                Ast::sub(
                    Ast::add(
                        Ast::mul(Ast::imm(6), Ast::arg(0)),
                        Ast::mul(Ast::imm(5), Ast::arg(1))
                    ),
                    Ast::mul(Ast::imm(3), Ast::arg(2))
                ),
                Ast::imm(8)
            )
        );
    }

    #[test]
    fn test_pass3_1() {
        let input = "[ x ] x + 2*5";
        let mut c = Compiler::new();
        let ast = c.pass1(input);
        let ast = c.pass2(&ast);
        let asm = c.pass3(&ast);

        assert_eq!(
            asm,
            vec![
                "IM 10".to_string(),
                "SW".to_string(),
                "AR 0".to_string(),
                "AD".to_string()
            ]
        );
    }

    #[test]
    fn test_pass3_2() {
        let input = "[ x y ] 6 * x + 5 * y";
        let mut c = Compiler::new();
        let ast = c.pass1(input);
        let ast = c.pass2(&ast);
        let asm = c.pass3(&ast);

        assert_eq!(simulate(asm, vec![4, 2]), 34);
    }

    #[test]
    fn test_pass3_3() {
        let input = "[ x ] 6 * ( x + 42 )";
        let mut c = Compiler::new();
        let ast = c.pass1(input);
        let ast = c.pass2(&ast);
        let asm = c.pass3(&ast);

        assert_eq!(simulate(asm, vec![8]), 300);
    }

    #[test]
    fn test_pass3_4() {
        let input = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";
        let mut c = Compiler::new();
        let ast = c.pass1(input);
        let asm = c.pass3(&ast);

        assert_eq!(simulate(asm, vec![4, 6, 2]), 48 / 8);
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
