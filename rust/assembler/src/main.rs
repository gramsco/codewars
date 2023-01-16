// https://www.codewars.com/kata/58e24788e24ddee28e000053/train/rust

use std::{collections::HashMap, convert::TryInto, vec};

enum Instruction {
    Move(String, i64),
    Jump(String, i64),
    Increment(String),
    Decrement(String),
}

fn parse(str: &str) -> Instruction {
    let s: Vec<&str> = str.split_whitespace().collect();
    let p1 = s.get(0);
    let p2 = s.get(1);
    let p3 = s.get(2);

    match (p1, p2, p3) {
        (Some(&"mov"), Some(x), Some(y)) => {
            Instruction::Move(x.to_string(), y.parse::<i64>().unwrap())
        }
        (Some(&"jnz"), Some(x), Some(y)) => {
            Instruction::Jump(x.to_string(), y.parse::<i64>().unwrap())
        }
        (Some(&"inc"), Some(x), None) => Instruction::Increment(x.to_string()),
        (Some(&"dec"), Some(x), None) => Instruction::Decrement(x.to_string()),

        _ => panic!("INVALID_SYNTAX {:?}", (p1, p2, p3)),
    }
}

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();

    let ite = program.iter();

    let mut passed_instructions: Vec<&str> = vec![];

    for instruction in ite {
        match parse(instruction) {
            Instruction::Move(x, n) => if let Some(_) = registers.insert(x, n) {},
            Instruction::Jump(x, quanto) => {
                if let Some(value) = registers.get(&x) {
                    if *value != 0 {
                        if quanto < 0 {
                            for &instruction2 in passed_instructions
                                .iter()
                                .rev()
                                .nth(quanto.abs().try_into().unwrap())
                            {
                                let v = simple_assembler(vec![instruction2]);
                                *value = value + v[&x];
                            }
                        } else {
                            ite.skip(quanto.try_into().unwrap());
                        }
                    }
                }
            }
            Instruction::Increment(x) => {
                let &a = registers.get(&x).unwrap();
                registers.insert(x, a + 1);
            }
            Instruction::Decrement(x) => {
                let &a = registers.get(&x).unwrap();
                registers.insert(x, a - 1);
            }
            _ => unreachable!(),
        }
        passed_instructions.push(&instruction);
    }
    registers
}

fn main() {
    let r = simple_assembler(vec![
        "mov a 5", "inc a", "dec a", "dec a", "inc a", "jnz a -1",
    ]);
    println!("{r:?}");
}
