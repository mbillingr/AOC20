#![allow(dead_code)]

use crate::intcode2::{Computer, Op, Operand};
use std::collections::{HashMap, HashSet};

fn build_block(ops: &[FixOp]) -> Vec<String> {
    use FixOp::*;
    let mut code = vec![];
    for op in ops {
        let c = match op {
            Inp(c) => format!("{} = inp();", build_operand(c)),
            Equ(a, b, c) => format!(
                "{} = {} == {};",
                build_operand(c),
                build_operand(a),
                build_operand(b)
            ),
            _ => unimplemented!("{:?}", op),
        };
        code.push(c);
    }
    code
}

fn build_operand(p: &Operand<i64>) -> String {
    match p {
        Operand::Imm(i) => i.to_string(),
        Operand::Pos(p) => format!("pos_{}", p),
        _ => unimplemented!(),
    }
}

fn find_used_labels(labels: Vec<usize>, ops: &[FixOp]) -> Vec<Option<usize>> {
    use Operand::*;
    let mut used = HashSet::new();
    used.insert(0);
    for op in ops {
        match op {
            FixOp::Jit(_, label) | FixOp::Jif(_, label) | FixOp::Jmp(label) => {
                used.insert(*label);
            }
            FixOp::Set(Imm(label), Rel(0)) => {
                used.insert(*label as usize);
            }
            _ => {}
        }
    }

    labels.into_iter().map(|l| used.get(&l).cloned()).collect()
}

fn cut_blocks(labels: Vec<Option<usize>>, ops: &[FixOp]) -> HashMap<usize, &[FixOp]> {
    let mut label = 0;
    let mut start = 0;

    let mut blocks = HashMap::new();

    for (i, l) in labels.iter().enumerate().skip(1) {
        if let Some(l) = l {
            blocks.insert(label, &ops[start..i]);
            start = i;
            label = *l;
        }
    }
    blocks.insert(label, &ops[start..]);

    blocks
}

fn sanitize_blocks(blocks: HashMap<usize, &[FixOp]>) -> HashMap<usize, Vec<FixOp>> {
    let mut block_labels: Vec<_> = blocks.keys().copied().collect();
    block_labels.sort();

    let mut result = HashMap::new();
    for i in 0..block_labels.len() {
        let label = block_labels[i];
        let mut code: Vec<_> = blocks[&label].iter().cloned().collect();

        while let Some(FixOp::Invalid) = code.last() {
            code.pop();
        }

        match code.last() {
            None => {}
            Some(FixOp::Halt) => {}
            Some(FixOp::Jr0) => {}
            Some(FixOp::Jmp(j)) if *j < i => {
                // assume a back-jump is always a loop
                let new = vec![FixOp::Loop].join(result.remove(j).unwrap());
                result.insert(*j, new);
            }
            Some(FixOp::Jmp(_)) => {}
            _ => code.push(FixOp::Jmp(block_labels[i + 1])),
        }

        result.insert(label, code);
    }

    result
}

fn rustify(current_label: usize, code: &[FixOp], blocks: &HashMap<usize, Vec<FixOp>>) -> String {
    use FixOp::*;

    if code.is_empty() {
        return String::new();
    }

    (match &code[0] {
        Loop => "loop {".to_string(),
        Halt => "halt();".to_string(),
        Inp(x) => format!("{} = input();", build_operand(x)),
        Out(a) => format!("output({});", build_operand(a)),
        Add(a, b, c) => format!(
            "{} = {} + {};",
            build_operand(c),
            build_operand(a),
            build_operand(b)
        ),
        Mul(a, b, c) => format!(
            "{} = {} * {};",
            build_operand(c),
            build_operand(a),
            build_operand(b)
        ),
        Equ(a, b, c) => format!(
            "{} = {} == {};",
            build_operand(c),
            build_operand(a),
            build_operand(b)
        ),
        Set(a, c) => format!("{} = {};", build_operand(c), build_operand(a)),
        Jit(a, label) => return rustify_branch(*a, "!=", *label, current_label, code, blocks),

        Jif(a, label) => return rustify_branch(*a, "==", *label, current_label, code, blocks),

        Dynamic(op) => format!("dynamic({:?})", op),

        Jmp(label) if *label > current_label => format!("JMP {}", label),
        Jmp(label) if *label < current_label => format!("}}  // {:?}", label),
        op => unimplemented!("{:?}", op),
    }) + "\n"
        + &rustify(current_label, &code[1..], blocks)
}

fn rustify_branch(
    operand: Operand<i64>,
    _cmp: &str,
    label: usize,
    current_label: usize,
    code: &[FixOp],
    blocks: &HashMap<usize, Vec<FixOp>>,
) -> String {
    use FixOp::*;

    let exit = if let Jmp(x) = blocks[&label].last().unwrap() {
        println!("{} -> {}", label, x);
        assert!(*x > label);
        *x
    } else {
        panic!("no jump at branch end");
    };
    let consequence = rustify(label, &blocks[&label], blocks);
    let alternative = rustify(current_label, &code[1..], blocks);
    let ifpart = format!(
        "if {} != 0 {{\n{}\n}} else {{\n{}\n}}",
        build_operand(&operand),
        consequence,
        alternative
    );

    ifpart + "\n" + &rustify(exit, &blocks[&exit], blocks)
}

fn transform(ops: Vec<FixOp>) -> Vec<FixOp> {
    use FixOp::*;
    use Operand::*;
    ops.into_iter()
        .map(|op| match op {
            Add(Imm(a), Imm(b), c) => Set(Imm(a + b), c),
            Mul(Imm(a), Imm(b), c) => Set(Imm(a * b), c),
            Add(a, Imm(0), c) | Add(Imm(0), a, c) => Set(a, c),
            Mul(a, Imm(1), c) | Mul(Imm(1), a, c) => Set(a, c),
            Jit(Imm(1), p) => Jmp(p as usize),
            Jif(Imm(0), p) => Jmp(p as usize),
            _ => op,
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CellType {
    Unknown,
    Constant,
    Mutable,
}

struct Analyzer {
    mem: Vec<CellType>,
    compiled: HashMap<usize, FixOp>,
    op_sizes: HashMap<usize, usize>,
    vm: Computer,
}

impl Analyzer {
    fn walk(&mut self) {
        loop {
            let pc = self.vm.pc;
            let (op, delta) = self.vm.peek().expect("invalid op");
            self.vm.pc += delta;

            if self.compiled.contains_key(&pc) {
                //ops.push(FixOp::Jmp(pc));
                println!("falling into compiled code...");
                return;
            }

            let mut dynamic = false;

            for i in 0..delta {
                if let Some(_) = self.mark_constant(Operand::Pos(pc + i)) {
                    dynamic = true;
                }
            }

            let fop = match op {
                _ if dynamic => match op {
                    Op::Jit(_, _) | Op::Jif(_, _) => {
                        println!("please, no dynamic jumps!");
                        FixOp::Dynamic(pc)
                    }
                    _ => FixOp::Dynamic(pc),
                },
                Op::Jit(Operand::Imm(1), Operand::Rel(0)) => FixOp::Jr0,
                Op::Jif(Operand::Imm(0), Operand::Rel(0)) => FixOp::Jr0,
                //Op::Jit(Operand::Imm(1), Operand::Imm(p)) => FixOp::Jmp(p as usize),
                //Op::Jif(Operand::Imm(0), Operand::Imm(p)) => FixOp::Jmp(p as usize),
                Op::Halt => FixOp::Halt,
                Op::Invalid => FixOp::Invalid,
                Op::Add(a, b, c) => FixOp::Add(a, b, c),
                Op::Mul(a, b, c) => FixOp::Mul(a, b, c),
                Op::Inp(c) => FixOp::Inp(c),
                Op::Out(a) => FixOp::Out(a),
                Op::Equ(a, b, c) => FixOp::Equ(a, b, c),
                Op::Ltn(a, b, c) => FixOp::Ltn(a, b, c),
                Op::Jit(a, Operand::Imm(b)) => FixOp::Jit(a, b as usize),
                Op::Jif(a, Operand::Imm(b)) => FixOp::Jif(a, b as usize),
                Op::Jit(_, _) | Op::Jif(_, _) => unimplemented!(),

                // unimplemented stuff... fall back to dynamic evaluation
                Op::Crb(_) => FixOp::Dynamic(pc),
                //Op::Crb(a) => FixOp::Crb(a),
            };

            //println!("{:4}  {:?}", pc, fop);

            self.compiled.insert(pc, fop.clone());
            self.op_sizes.insert(pc, delta);

            match fop {
                FixOp::Halt | FixOp::Jr0 | FixOp::Invalid => return,
                FixOp::Out(_) | FixOp::Dynamic(_) => {}
                FixOp::Set(_, c)
                | FixOp::Add(_, _, c)
                | FixOp::Mul(_, _, c)
                | FixOp::Inp(c)
                | FixOp::Equ(_, _, c)
                | FixOp::Ltn(_, _, c) => {
                    if let Some(p) = self.mark_mutable(c) {
                        self.force_mut(p)
                    }
                }
                FixOp::Jmp(p) => {
                    self.vm.pc = p;
                }
                FixOp::Jit(_, b) | FixOp::Jif(_, b) => {
                    //assert!(self.mark_constant(b).is_none());
                    let vm = self.vm.clone();
                    self.vm.pc = b;
                    let _branch = self.walk();
                    self.vm = vm;
                }
                FixOp::Crb(a) => {
                    self.vm.rel_base += self.get(a) as isize;
                }

                FixOp::Unknown => println!("ignoring {:?}", fop),
                _ => unimplemented!("{:?}", fop),
            }
        }
    }

    fn force_mut(&mut self, pos: usize) {
        let p = *self.compiled.keys().filter(|&&k| k <= pos).max().unwrap();
        let fop = self.compiled.get_mut(&p).unwrap();

        if pos >= p && pos < p + self.op_sizes[&p] {
            *fop = FixOp::Dynamic(p);
        }
    }

    fn get(&self, o: Operand<i64>) -> i64 {
        match o {
            Operand::Imm(i) => i,
            Operand::Pos(_) => unimplemented!(),
            Operand::Rel(_) => unimplemented!(),
            Operand::Push | Operand::Pop => unimplemented!(),
        }
    }

    fn mark_constant(&mut self, x: Operand<i64>) -> Option<usize> {
        match x {
            Operand::Imm(_) => {}
            Operand::Pos(p) => {
                if self.mem[p] == CellType::Mutable {
                    //eprintln!("Memory location {} cannot be both, mutable and constant", p);
                    return Some(p);
                } else {
                    self.mem[p] = CellType::Constant;
                }
            }
            Operand::Rel(r) => {
                self.mem[(self.vm.rel_base as isize + r) as usize] = CellType::Constant
            }
            Operand::Push | Operand::Pop => unimplemented!(),
        }
        None
    }

    fn mark_mutable(&mut self, x: Operand<i64>) -> Option<usize> {
        match x {
            Operand::Imm(_) => {}
            Operand::Pos(p) => {
                if self.mem[p] == CellType::Constant {
                    self.mem[p] = CellType::Mutable;
                    //eprintln!("Memory location {} cannot be both, mutable and constant", p);
                    return Some(p);
                } else {
                    self.mem[p] = CellType::Mutable;
                }
            }
            Operand::Rel(r) => {
                self.mem[(self.vm.rel_base as isize + r) as usize] = CellType::Mutable
            }
            Operand::Push | Operand::Pop => unimplemented!(),
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum FixOp {
    Invalid,
    Halt,
    Add(Operand<i64>, Operand<i64>, Operand<i64>),
    Mul(Operand<i64>, Operand<i64>, Operand<i64>),
    Inp(Operand<i64>),
    Out(Operand<i64>),
    Jit(Operand<i64>, usize),
    Jif(Operand<i64>, usize),
    Ltn(Operand<i64>, Operand<i64>, Operand<i64>),
    Equ(Operand<i64>, Operand<i64>, Operand<i64>),
    Crb(Operand<i64>),

    Set(Operand<i64>, Operand<i64>),
    Jmp(usize),
    Jr0,
    Dynamic(usize),
    Loop,
    Unknown,
}

trait Join {
    fn join(self, rhs: Self) -> Self;
}

impl<T> Join for Vec<T> {
    fn join(mut self, rhs: Self) -> Self {
        self.extend(rhs);
        self
    }
}
