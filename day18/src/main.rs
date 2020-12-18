use common::input::Input;
use common::itertools::__std_iter::Peekable;
use std::str::Chars;

fn main() {
    let input = Input::from_file("data/day18-input.txt");

    let total: u64 = input
        .iter_lines()
        .map(LeftFirstParser::parse_str)
        .map(|exp| exp.eval())
        .sum();

    println!("Part 1: {}", total);

    let total: u64 = input
        .iter_lines()
        .map(AddFirstParser::parse_str)
        .map(|exp| exp.eval())
        .sum();

    println!("Part 2: {}", total);
}

#[derive(Debug, Clone)]
enum Expression {
    Number(u64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> u64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Add(l, r) => l.eval() + r.eval(),
            Expression::Mul(l, r) => l.eval() * r.eval(),
        }
    }
}

struct LeftFirstParser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> for LeftFirstParser<'a> {
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }
    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn parse_str(s: &'a str) -> Expression {
        let input = s.chars().peekable();
        let mut parser = LeftFirstParser { input };
        parser.parse_expression()
    }

    fn parse_expression(&mut self) -> Expression {
        let mut left = self.parse_operand();
        loop {
            match self.peek() {
                Some(')') => {
                    self.next().unwrap();
                    return left;
                }
                None => return left,
                _ => {}
            }
            left = self.parse_operator(left);
        }
    }
}

impl<'a> LeftFirstParser<'a> {
    fn parse_operator(&mut self, left: Expression) -> Expression {
        self.skip_whitespace();
        match self.peek() {
            Some('+') => {
                self.next().unwrap();
                let right = self.parse_operand();
                Expression::Add(Box::new(left), Box::new(right))
            }
            Some('*') => {
                self.next().unwrap();
                let right = self.parse_operand();
                Expression::Mul(Box::new(left), Box::new(right))
            }
            ch => unimplemented!("{:?}", ch),
        }
    }
}

struct AddFirstParser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> for AddFirstParser<'a> {
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }
    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn parse_str(s: &'a str) -> Expression {
        let input = s.chars().peekable();
        let mut parser = Self { input };
        parser.parse_expression()
    }

    fn parse_expression(&mut self) -> Expression {
        let mut sy = ShuntingYard::new();

        loop {
            self.skip_whitespace();
            match self.peek() {
                None => break,
                Some(')') => {
                    self.next().unwrap();
                    break;
                }
                Some('+') | Some('*') => sy.push_operator(self.next().unwrap()),
                _ => sy.push_operand(self.parse_operand()),
            }
        }

        sy.finalize()
    }
}

impl<'a> AddFirstParser<'a> {}

struct ShuntingYard {
    output_stack: Vec<Expression>,
    operator_stack: Vec<char>,
}

impl ShuntingYard {
    fn new() -> Self {
        ShuntingYard {
            output_stack: vec![],
            operator_stack: vec![],
        }
    }

    fn finalize(mut self) -> Expression {
        while !self.operator_stack.is_empty() {
            self.pop_operator()
        }

        assert_eq!(self.output_stack.len(), 1);

        self.output_stack.pop().unwrap()
    }

    fn push_operand(&mut self, expr: Expression) {
        self.output_stack.push(expr)
    }

    fn push_operator(&mut self, ch: char) {
        match (self.operator_stack.last(), ch) {
            (Some(l), r) if !Self::precedes(r, *l) => self.pop_operator(),
            _ => {}
        }
        self.operator_stack.push(ch)
    }

    fn pop_operator(&mut self) {
        let right = self.output_stack.pop().unwrap();
        let left = self.output_stack.pop().unwrap();

        let ch = self.operator_stack.pop().unwrap();
        match ch {
            '+' => self
                .output_stack
                .push(Expression::Add(Box::new(left), Box::new(right))),
            '*' => self
                .output_stack
                .push(Expression::Mul(Box::new(left), Box::new(right))),
            _ => unreachable!(),
        }
    }

    fn precedes(op: char, other: char) -> bool {
        match (op, other) {
            ('+', '*') => true,
            _ => false,
        }
    }
}

trait Parser<'a> {
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;

    fn parse_str(s: &'a str) -> Expression;
    fn parse_expression(&mut self) -> Expression;

    fn skip_whitespace(&mut self) {
        while self.peek().filter(|ch| ch.is_whitespace()).is_some() {
            self.next();
        }
    }

    fn parse_operand(&mut self) -> Expression {
        self.skip_whitespace();
        match self.peek() {
            Some('(') => {
                self.next().unwrap();
                self.parse_expression()
            }
            Some(ch) if ch.is_numeric() => self.parse_number(),
            ch => unimplemented!("{:?}", ch),
        }
    }

    fn parse_number(&mut self) -> Expression {
        let ch = self.next().unwrap();

        let n = match ch {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => unreachable!(),
        };

        assert!(!self.peek().filter(|ch| ch.is_numeric()).is_some());

        self.skip_whitespace();

        Expression::Number(n)
    }
}
