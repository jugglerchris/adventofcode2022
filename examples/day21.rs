use std::{collections::HashMap, fmt::Debug};

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

type Val = isize;

#[derive(Clone, Debug)]
pub struct Monkey {
    name: String,
    op: Op,
}

#[derive(Clone, Debug)]
pub enum Op {
    Const(Val),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Op {
    fn is_const(&self) -> bool {
        match self {
            Op::Const(_) => true,
            _ => false,
        }
    }
}

regex_parser!(parse_monkey: Monkey {
    C = r#"(\w+): (-?\d+)$"# =>
        |name: String, c: Val| Monkey {
            name, op: Op::Const(c)
        },
    ADD = r#"(\w+): (\w+) \+ (\w+)$"# =>
        |name: String, a: String, b: String| Monkey {
            name, op: Op::Add(a, b)
        },
    SUB = r#"(\w+): (\w+) - (\w+)$"# =>
        |name: String, a: String, b: String| Monkey {
            name, op: Op::Sub(a, b)
        },
    MUL = r#"(\w+): (\w+) \* (\w+)$"# =>
        |name: String, a: String, b: String| Monkey {
            name, op: Op::Mul(a, b)
        },
    DIV = r#"(\w+): (\w+) / (\w+)$"# =>
        |name: String, a: String, b: String| Monkey {
            name, op: Op::Div(a, b)
        }
});

type Data = Vec<Monkey>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

timeit!{
fn part1(data: &Data) -> isize {
    let mut monkeys: HashMap<String, Op> =
        data.iter()
            .map(|m| (m.name.clone(), m.op.clone()))
            .collect();

    let mut queue = vec![String::from("root")];
    while !queue.is_empty() {
        let top = queue.pop().unwrap();
        let monkey = monkeys.get(&top).unwrap();
        match monkey {
            Op::Const(_) => continue,
            Op::Add(a, b) => {
                let opa = monkeys.get(a.as_str()).unwrap();
                let opb = monkeys.get(b.as_str()).unwrap();
                if let (Op::Const(aa), Op::Const(bb)) = (opa, opb) {
                    *monkeys.get_mut(&top).unwrap() = Op::Const(aa + bb);
                } else {
                    queue.push(top);
                    if !opa.is_const() {
                        queue.push(a.clone());
                    }
                    if !opb.is_const() {
                        queue.push(b.clone());
                    }
                }
            },
            Op::Sub(a, b) => {
                let opa = monkeys.get(a.as_str()).unwrap();
                let opb = monkeys.get(b.as_str()).unwrap();
                if let (Op::Const(aa), Op::Const(bb)) = (opa, opb) {
                    *monkeys.get_mut(&top).unwrap() = Op::Const(aa - bb);
                } else {
                    queue.push(top);
                    if !opa.is_const() {
                        queue.push(a.clone());
                    }
                    if !opb.is_const() {
                        queue.push(b.clone());
                    }
                }
            },
            Op::Mul(a, b) => {
                let opa = monkeys.get(a.as_str()).unwrap();
                let opb = monkeys.get(b.as_str()).unwrap();
                if let (Op::Const(aa), Op::Const(bb)) = (opa, opb) {
                    *monkeys.get_mut(&top).unwrap() = Op::Const(aa * bb);
                } else {
                    queue.push(top);
                    if !opa.is_const() {
                        queue.push(a.clone());
                    }
                    if !opb.is_const() {
                        queue.push(b.clone());
                    }
                }
            },
            Op::Div(a, b) => {
                let opa = monkeys.get(a.as_str()).unwrap();
                let opb = monkeys.get(b.as_str()).unwrap();
                if let (Op::Const(aa), Op::Const(bb)) = (opa, opb) {
                    *monkeys.get_mut(&top).unwrap() = Op::Const(*aa / *bb);
                } else {
                    queue.push(top);
                    if !opa.is_const() {
                        queue.push(a.clone());
                    }
                    if !opb.is_const() {
                        queue.push(b.clone());
                    }
                }
            },
        }
    }
    if let Op::Const(n) = monkeys.get("root").unwrap() {
        *n
    } else {
        panic!("{:?}", monkeys.get("root"));
    }
}}

#[derive(Clone)]
pub enum InlineOp {
    Const(Val),
    Human,
    Add(Box<InlineOp>, Box<InlineOp>),
    Sub(Box<InlineOp>, Box<InlineOp>),
    Mul(Box<InlineOp>, Box<InlineOp>),
    Div(Box<InlineOp>, Box<InlineOp>),
}

impl Debug for InlineOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(arg0) => write!(f, "{}", arg0),
            Self::Human => write!(f, "Human"),
            Self::Add(arg0, arg1) => write!(f, "({:?}+{:?})", arg0, arg1),
            Self::Sub(arg0, arg1) => write!(f, "({:?}-{:?})", arg0, arg1),
            Self::Mul(arg0, arg1) => write!(f, "({:?}*{:?})", arg0, arg1),
            Self::Div(arg0, arg1) => write!(f, "({:?}/{:?})", arg0, arg1),
        }
    }
}

impl InlineOp {
    fn has_human(&self) -> bool {
        match self {
            InlineOp::Const(_) => false,
            InlineOp::Human => true,
            InlineOp::Add(a, b) |
            InlineOp::Sub(a, b) |
            InlineOp::Mul(a, b) |
            InlineOp::Div(a, b) => a.has_human() || b.has_human(),
        }
    }
    fn simplify(&self) -> Self {
        match self {
            InlineOp::Const(n) => InlineOp::Const(*n),
            InlineOp::Human => InlineOp::Human,
            InlineOp::Add(a, b) |
            InlineOp::Sub(a, b) |
            InlineOp::Mul(a, b) |
            InlineOp::Div(a, b) => {
                let aval = a.simplify();
                let bval = b.simplify();
                if aval.is_const() && bval.is_const() {
                    let val = match self {
                        InlineOp::Add(_, _) => aval.get_const() + bval.get_const(),
                        InlineOp::Sub(_, _) => aval.get_const() - bval.get_const(),
                        InlineOp::Mul(_, _) => aval.get_const() * bval.get_const(),
                        InlineOp::Div(_, _) => aval.get_const() / bval.get_const(),
                        _ => panic!(),
                    };
                    InlineOp::Const(val)
                } else {
                    match self {
                        InlineOp::Add(_, _) => InlineOp::Add(Box::new(aval), Box::new(bval)),
                        InlineOp::Sub(_, _) => InlineOp::Sub(Box::new(aval), Box::new(bval)),
                        InlineOp::Mul(_, _) => InlineOp::Mul(Box::new(aval), Box::new(bval)),
                        InlineOp::Div(_, _) => InlineOp::Div(Box::new(aval), Box::new(bval)),
                        _ => panic!(),
                    }
                }
            }
        }
    }
    fn eval(&self) -> Val {
        match self {
            InlineOp::Const(n) => *n,
            InlineOp::Human => panic!(),
            InlineOp::Add(a, b) => a.eval() + b.eval(),
            InlineOp::Sub(a, b) => a.eval() - b.eval(),
            InlineOp::Mul(a, b) => a.eval() * b.eval(),
            InlineOp::Div(a, b) => a.eval() / b.eval(),
        }
    }
    fn eval_with_human(&self, hv: Val) -> Val {
        match self {
            InlineOp::Const(n) => *n,
            InlineOp::Human => hv,
            InlineOp::Add(a, b) => a.eval_with_human(hv) + b.eval_with_human(hv),
            InlineOp::Sub(a, b) => a.eval_with_human(hv) - b.eval_with_human(hv),
            InlineOp::Mul(a, b) => a.eval_with_human(hv) * b.eval_with_human(hv),
            InlineOp::Div(a, b) => a.eval_with_human(hv) / b.eval_with_human(hv),
        }
    }
    fn is_const(&self) -> bool {
        match self {
            InlineOp::Const(_) => true,
            _ => false,
        }
    }
    fn get_const(&self) -> Val {
        match self {
            InlineOp::Const(n) => *n,
            _ => panic!(),
        }
    }
}

fn get_inline_op(monkeys: &HashMap<String, Op>, name: &str) -> Box<InlineOp> {
    let monkey = monkeys.get(name).unwrap();
    if name == "humn" {
        Box::new(InlineOp::Human)
    } else {
        Box::new(match monkey {
            Op::Const(n) => InlineOp::Const(*n),
            Op::Add(a, b) => InlineOp::Add(
                get_inline_op(monkeys, a),
                get_inline_op(monkeys, b)),
            Op::Sub(a, b) => InlineOp::Sub(
                get_inline_op(monkeys, a),
                get_inline_op(monkeys, b)),
            Op::Mul(a, b) => InlineOp::Mul(
                get_inline_op(monkeys, a),
                get_inline_op(monkeys, b)),
            Op::Div(a, b) => InlineOp::Div(
                get_inline_op(monkeys, a),
                get_inline_op(monkeys, b)),
        })
    }
}

timeit!{
fn part2(data: &Data) -> isize {
    let mut monkeys: HashMap<String, Op> =
        data.iter()
            .map(|m| (m.name.clone(), m.op.clone()))
            .collect();

    let (a_name, b_name): (String, String) =
           match monkeys.get_mut("root").unwrap() {
               Op::Add(a, b) => (a.clone(), b.clone()),
               _ => panic!(),
           };
    let a = get_inline_op(&monkeys, &a_name);
    let b = get_inline_op(&monkeys, &b_name);

    let (c, hexpr) = if a.has_human() {
        (b.eval(), a)
    } else {
        (a.eval(), b)
    };
    dbg!(hexpr.simplify());
    dbg!(&c);

    let mut hval = 0;
    let mut inc = 0x1000;


    let with0 = hexpr.eval_with_human(0);
    let with1 = hexpr.eval_with_human(inc);
    let pos = if with0 < with1 {
        true
    } else if with0 > with1 {
        false
    } else {
        panic!()
    };
    loop {
        inc *= 2;
        let v0 = hexpr.eval_with_human(hval);
        let v1 = hexpr.eval_with_human(hval + inc);
        if (pos && v1 > c) || (!pos && v1 < c) {
            break;
        }
    }
    loop {
        let v1 = hexpr.eval_with_human(hval + inc);

        if v1 == c {
            let mut hv = hval + inc;
            while hexpr.eval_with_human(hv-1) == c {
                hv -= 1;
            }
            return hv;
        } else if (pos && v1 < c) || (!pos && v1 > c) {
            hval += inc;
        }
        inc /= 2;
        if inc == 0 {
            panic!();
        }
    }
    unreachable!()
}}

#[test]
fn test() {
    let tests = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 152);
    assert_eq!(part2(&data), 301);
}

fn main() -> std::io::Result<()>{
    let input = get_input(21)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
