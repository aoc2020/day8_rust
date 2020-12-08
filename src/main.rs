use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;


fn main() {
    let lines = read_lines("input.txt");
    if lines.is_err() {
        println!("Error reading file");
    } else {
        let regex = Regex::new(r"([a-z]{3}) ([+-]\d+)").unwrap();
        let parse = |s:String| -> Inst {
            let capt = regex.captures(&s).unwrap();
            let op_str = capt.get(1).unwrap().as_str().clone();
            let op = String::from(op_str);
            let param_s : &str = capt.get(2).unwrap().as_str();
            println!("[{}] [{}]", op, param_s);
            let param : i32 = param_s.parse().unwrap();
            Inst::new(op,param)
        };
        let program: Vec<Inst> = lines.unwrap().into_iter().map(|s| parse(s.unwrap())).collect();
        let vm = VM::new(program);
        let vm1 = vm.exec();
        println!("Answer 1: {}",vm1.acc);
        let vm2 = vm.find_ok_and_run();
        println!("Ansert 2: {}", vm2.acc);
    }
//    println!("Answer 2 : {}", valids2);
}

#[derive(Clone)]
struct Inst {
    op: String,
    param: i32
}

impl Inst {
    fn new (op: String, param: i32) -> Inst {
        Inst { op, param }
    }
    fn clone(&self) -> Inst {
        Inst::new(self.op.clone(), self.param)
    }
    fn replace(&self) -> Inst {
        match self.op.clone() {
            i if i == "jmp" => Inst::new(String::from("nop"),self.param),
            i if i == "nop" => Inst::new(String::from("jmp"),self.param),
            i => self.clone()
        }
    }
}

struct VM {
    program: Vec<Inst>,
    ip: i32,
    acc: i32,
    visited: HashSet<i32>
}

impl VM {
    fn visit(&self) -> HashSet<i32> {
        let mut visited = self.visited.clone();
        visited.insert(self.ip);
        visited
    }
    fn new(program: Vec<Inst>) -> VM {
        VM { program, ip:0, acc:0, visited: HashSet::new() }
    }
    fn jmp(&self, step: i32) -> VM {
        VM { program: self.program.clone(), ip: self.ip + step, acc: self.acc, visited: self.visit() }
    }
    fn acc(&self, value: i32) -> VM {
        VM { program: self.program.clone(), ip: self.ip + 1, acc: self.acc + value, visited: self.visit() }
    }
    fn nop(&self) -> VM {
        self.jmp(1)
    }
    fn ended(&self) -> bool {
        if self.visited.contains(&self.ip) {
            return true;
        }
        if self.ip < 0 {
            return true;
        }
        if self.ip >= (self.program.len() as i32) {
            return true;
        }
        false
    }
    fn ended_ok(&self) -> bool {
        self.ip == self.program.len() as i32
    }
    fn inst(&self) -> Inst {
        self.program[self.ip as usize].clone()
    }
    fn exec_inst(&self) -> VM {
        match self.inst() {
            i if i.op == "nop" => self.nop(),
            i if i.op == "jmp" => self.jmp(i.param),
            i if i.op == "acc" => self.acc(i.param),
            i => panic!(format!("Unknown instruction: {}", i.op))
        }
    }
    fn exec(&self) -> VM {
        let mut vm = self.exec_inst();
        while !vm.ended() {
            vm = vm.exec_inst();
        }
        vm
    }
    fn replace(&self, i: i32) -> VM {
        let mut program = self.program.clone();
        program[i as usize] = program[i as usize].clone().replace();
        VM::new(program)
    }

    fn find_ok_and_run(&self) -> VM {
        for i in 0..self.program.len() {
            let vm = self.replace(i as i32);
            let ended = vm.exec();
            if ended.ended_ok() {
                return ended
            }
        }
        panic!("No valid vm found")
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}