use std::collections::VecDeque;

pub fn run() -> String {
    let input = include_str!("../../inputs/day17.txt");
    solve(input)
}

struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    program: Vec<u8>,
    instruction_pointer: usize,
    output: VecDeque<u8>,
}

impl Computer {
    fn new(a: i32, b: i32, c: i32, program: Vec<u8>) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            program,
            instruction_pointer: 0,
            output: VecDeque::new(),
        }
    }

    fn get_combo_value(&self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("operand combo not valid: {}", operand),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            match opcode {
                0 => { // adv: divide register a by 2^combo_value, store result in a
                    let denominator = 2_i32.pow(self.get_combo_value(operand) as u32);
                    self.register_a /= denominator;
                }
                1 => { // bxl: b XOR literal operand -> b
                    self.register_b ^= operand as i32;
                }
                2 => { // bst: b = combo_value % 8
                    self.register_b = self.get_combo_value(operand) % 8;
                }
                3 => { // jnz: if a != 0, jump to literal operand
                    if self.register_a != 0 {
                        self.instruction_pointer = operand as usize;
                        continue; // Avoid increasing ip by 2
                    }
                }
                4 => { // bxc: b XOR c -> b (operand ignored)
                    self.register_b ^= self.register_c;
                }
                5 => { // out: output combo_value % 8
                    let value = (self.get_combo_value(operand) % 8) as u8;
                    self.output.push_back(value);
                }
                6 => { // bdv: divide a by 2^combo_value, store result in b
                    let denominator = 2_i32.pow(self.get_combo_value(operand) as u32);
                    self.register_b = self.register_a / denominator;
                }
                7 => { // cdv: divide a by 2^combo_value, store result in c
                    let denominator = 2_i32.pow(self.get_combo_value(operand) as u32);
                    self.register_c = self.register_a / denominator;
                }
                _ => panic!("unknown opcode: {}", opcode),
            }

            //next instruction
            self.instruction_pointer += 2;
        }
    }

    // get the output as a comma-separated string
    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|&computed_output| computed_output.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn solve(_input: &str) -> String {
    let register_a = 53437164;
    let register_b = 0;
    let register_c = 0;
    let program = vec![2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0];

    let mut computer = Computer::new(register_a, register_b, register_c, program);
    computer.run();

    computer.get_output()
}
