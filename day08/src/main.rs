use common::input::Input;

fn main() {
    let input = Input::from_file("data/day08-input.txt");

    let code = input.iter_lines().map(Op::from_str).collect::<Vec<_>>();

    let mut vm = Vm::new(code.clone());
    while vm.execution_counts[vm.ip] == 0 {
        vm.step();
    }

    println!("Part 1: {}", vm.accumulator);

    let mut final_acc = None;

    'search: for i in 0..code.len() {
        let mut mod_code = code.clone();
        mod_code[i] = swap_op(code[i]);

        let mut vm = Vm::new(mod_code.clone());
        loop {
            if vm.ip >= code.len() {
                final_acc = Some(vm.accumulator);
                break 'search;
            }

            if vm.execution_counts[vm.ip] > 0 {
                break;
            }

            vm.step();
        }
    }

    println!("Part 2: {}", final_acc.unwrap());
}

fn swap_op(op: Op) -> Op {
    match op {
        Op::Acc(arg) => Op::Acc(arg),
        Op::Nop(arg) => Op::Jmp(arg),
        Op::Jmp(arg) => Op::Nop(arg),
    }
}

type Argument = isize;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Vm {
    code: Vec<Op>,
    execution_counts: Vec<usize>,

    accumulator: Argument,
    ip: usize,
}

impl Vm {
    pub fn new(ops: Vec<Op>) -> Self {
        Vm {
            accumulator: 0,
            ip: 0,
            execution_counts: vec![0; ops.len()],
            code: ops,
        }
    }

    fn step(&mut self) {
        self.execution_counts[self.ip] += 1;
        let op = self.code[self.ip];
        self.ip += 1;
        self.dispatch(op);
    }

    fn dispatch(&mut self, op: Op) {
        match op {
            Op::Nop(_) => {}
            Op::Acc(arg) => self.accumulator += arg,
            Op::Jmp(ofs) => self.ip = (self.ip as isize - 1 + ofs as isize) as usize,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Acc(Argument),
    Jmp(Argument),
    Nop(Argument),
}

impl Op {
    pub fn from_str(instruction: &str) -> Self {
        let operation = &instruction[..3];
        let argument = &instruction[4..];
        let arg = argument.parse().unwrap();
        match operation {
            "nop" => Op::Nop(arg),
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg),
            _ => panic!("Unknown operation: {}", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nop() {
        assert_eq!(Op::from_str("nop +0"), Op::Nop(0))
    }

    #[test]
    fn parse_acc_positive() {
        assert_eq!(Op::from_str("acc +1"), Op::Acc(1))
    }

    #[test]
    fn parse_acc_negative() {
        assert_eq!(Op::from_str("acc -1"), Op::Acc(-1))
    }

    #[test]
    fn parse_jmp() {
        assert_eq!(Op::from_str("jmp 0"), Op::Jmp(0))
    }

    #[test]
    fn construct_vm() {
        let vm = Vm::new(vec![Op::Nop(0)]);
        assert_eq!(vm.execution_counts, vec![0]);
    }

    #[test]
    fn vm_step() {
        let vm0 = Vm::new(vec![Op::Nop(0)]);

        let mut vm = vm0.clone();
        vm.step();

        assert_eq!(vm.ip, 1);
    }

    #[test]
    fn dispatch_nop() {
        let vm0 = Vm::new(vec![]);
        let mut vm1 = vm0.clone();
        vm1.dispatch(Op::Nop(1));
        assert_eq!(vm1, vm0);
    }

    #[test]
    fn dispatch_acc() {
        let vm0 = Vm::new(vec![]);

        let arg = -1;

        let mut vm1 = vm0.clone();
        vm1.dispatch(Op::Acc(arg));

        let vm_expect = Vm {
            accumulator: vm0.accumulator + arg,
            ..vm0
        };

        assert_eq!(vm1, vm_expect);
    }

    #[test]
    fn dispatch_jmp() {
        let mut vm0 = Vm::new(vec![]);
        vm0.ip = 2;

        let mut vm1 = vm0.clone();
        vm1.dispatch(Op::Jmp(0));

        let vm_expect = Vm { ip: 1, ..vm0 };

        assert_eq!(vm1, vm_expect);
    }

    #[test]
    fn vm_step_jmp_modifies_ip_correctly() {
        let mut vm0 = Vm::new(vec![Op::Jmp(2)]);
        vm0.ip = 0;

        let mut vm = vm0.clone();
        vm.step();

        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn vm_step_increases_execution_counts() {
        let mut vm = Vm::new(vec![Op::Nop(0), Op::Nop(0)]);

        vm.step();

        assert_eq!(vm.execution_counts, vec![1, 0]);
    }
}
