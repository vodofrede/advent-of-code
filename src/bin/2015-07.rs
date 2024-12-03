use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/2015-07.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
enum Expr {
    Atom(u16),
    Ref(String),
    Cons(String, Vec<Expr>),
}
impl Expr {
    fn new(s: &str) -> Expr {
        if let Ok(n) = s.parse::<u16>() {
            Expr::Atom(n)
        } else {
            Expr::Ref(s.to_string())
        }
    }
}

fn part1(input: &str) {
    // parse registers
    let expr_re = Regex::new(r"(?:(\S+) )?(?:(AND|OR|LSHIFT|RSHIFT) )?(\S+)").unwrap();
    let op_re = Regex::new(r"(.+) -> ([a-z]+)").unwrap();
    let mut regs = input
        .lines()
        .filter_map(|l| op_re.captures(l))
        .map(|c| {
            let name = c[2].to_string();
            let c = expr_re.captures(&c[1]).unwrap();
            let (a, b, c) = (c.get(1), c.get(2), c.get(3));
            let expr = match (a, b, c) {
                (Some(op), None, Some(a)) => {
                    Expr::Cons(op.as_str().to_string(), vec![Expr::new(a.as_str())])
                }
                (Some(a), Some(op), Some(b)) => Expr::Cons(
                    op.as_str().to_string(),
                    vec![Expr::new(a.as_str()), Expr::new(b.as_str())],
                ),
                (None, None, Some(a)) => Expr::new(a.as_str()),
                _ => panic!(),
            };

            (name, expr)
        })
        .collect::<HashMap<_, _>>();

    // eval expressions
    fn eval(expr: Expr, regs: &mut HashMap<String, Expr>) -> u16 {
        match expr {
            Expr::Atom(n) => n,
            Expr::Ref(s) => {
                let expr = eval(regs[&s].clone(), regs);
                regs.insert(s, Expr::Atom(expr));
                expr
            }
            Expr::Cons(op, args) => match op.as_str() {
                "AND" => eval(args[0].clone(), regs) & eval(args[1].clone(), regs),
                "OR" => eval(args[0].clone(), regs) | eval(args[1].clone(), regs),
                "LSHIFT" => eval(args[0].clone(), regs) << eval(args[1].clone(), regs),
                "RSHIFT" => eval(args[0].clone(), regs) >> eval(args[1].clone(), regs),
                "NOT" => !eval(args[0].clone(), regs),
                _ => panic!(),
            },
        }
    }

    let value = eval(regs["a"].clone(), &mut regs);
    println!("part 1: {value}");
}
fn part2(input: &str) {
    // parse registers
    let expr_re = Regex::new(r"(?:(\S+) )?(?:(AND|OR|LSHIFT|RSHIFT) )?(\S+)").unwrap();
    let op_re = Regex::new(r"(.+) -> ([a-z]+)").unwrap();
    let mut regs = input
        .lines()
        .filter_map(|l| op_re.captures(l))
        .map(|c| {
            let name = c[2].to_string();
            let c = expr_re.captures(&c[1]).unwrap();
            let (a, b, c) = (c.get(1), c.get(2), c.get(3));
            let expr = match (a, b, c) {
                (Some(op), None, Some(a)) => {
                    Expr::Cons(op.as_str().to_string(), vec![Expr::new(a.as_str())])
                }
                (Some(a), Some(op), Some(b)) => Expr::Cons(
                    op.as_str().to_string(),
                    vec![Expr::new(a.as_str()), Expr::new(b.as_str())],
                ),
                (None, None, Some(a)) => Expr::new(a.as_str()),
                _ => panic!(),
            };

            (name, expr)
        })
        .collect::<HashMap<_, _>>();

    // eval expressions
    fn eval(expr: Expr, regs: &mut HashMap<String, Expr>) -> u16 {
        match expr {
            Expr::Atom(n) => n,
            Expr::Ref(s) => {
                let expr = eval(regs[&s].clone(), regs);
                regs.insert(s, Expr::Atom(expr));
                expr
            }
            Expr::Cons(op, args) => match op.as_str() {
                "AND" => eval(args[0].clone(), regs) & eval(args[1].clone(), regs),
                "OR" => eval(args[0].clone(), regs) | eval(args[1].clone(), regs),
                "LSHIFT" => eval(args[0].clone(), regs) << eval(args[1].clone(), regs),
                "RSHIFT" => eval(args[0].clone(), regs) >> eval(args[1].clone(), regs),
                "NOT" => !eval(args[0].clone(), regs),
                _ => panic!(),
            },
        }
    }

    let a = eval(regs["a"].clone(), &mut regs.clone());
    regs.insert("b".to_string(), Expr::Atom(a));
    let b = eval(regs["a"].clone(), &mut regs);
    println!("part 2: {b}");
}
