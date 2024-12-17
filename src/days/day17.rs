use std::collections::VecDeque;

pub fn run() -> String {
    let input = include_str!("../../inputs/day17.txt");
    solve(input)
}

struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
    instruction_pointer: usize,
    output: VecDeque<u8>,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, program: Vec<u8>) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            program,
            instruction_pointer: 0,
            output: VecDeque::new(),
        }
    }

    fn get_combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
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
                    let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
                    self.register_a /= denominator;
                }
                1 => { // bxl: b XOR literal operand -> b
                    self.register_b ^= operand as i64;
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
                    let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
                    self.register_b = self.register_a / denominator;
                }
                7 => { // cdv: divide a by 2^combo_value, store result in c
                    let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
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
    // Initial conditions
    let initial_b = 0;
    let initial_c = 0;
    let program = vec![2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0];
    let mut a = 104370664219872; // start search
    // lowest 16: 35185000000000
    let increment = 42991616; //  8
    let mut prev = 0;

    loop {
        // create and run the computer with the current a value
        let mut computer = Computer::new(a, initial_b, initial_c, program.clone());
        computer.run();
        let output = computer.get_output();

        if computer.output.iter().take(7).eq([2,4,1,7,7,5,4].iter()) {
            println!("current a: {}, len(output): {}, output: {}, delta: {}", a, output.len(), output, a - prev);
            println!("target: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0");
            prev = a;
        }

        // convert program to a comma-separated string
        let program_str = program.iter().map(|&byte| byte.to_string()).collect::<Vec<_>>().join(",");

        // check if output matches the program
        if output == program_str {
            println!("Found the lowest A: {}", a);
            break;
        }

        a += increment;
    }
    /* let register_b = 0;
    let register_c = 0;
    let program = vec![2, 4, 1, 7, 7, 5, 4, 1, 1, 4, 5, 5, 0, 3, 3, 0];
    // let program = vec![0,3,5,4,3,0];

    // Convert program to a comma-separated string
    let program_str = program.iter().map(|&byte| byte.to_string()).collect::<Vec<_>>().join(",");

    let starting_register_a: i64 = 70368745000000; // 40000000000000;

    for register_a in starting_register_a.. {
        let mut computer = Computer::new(register_a, register_b, register_c, program.clone());
        computer.run();
        let output = computer.get_output();


        if output == program_str {
            return register_a.to_string();
        }

        if register_a % 100000 == 0 {
            println!("ran {}", register_a);
            
            println!("target:  {}", program_str);
            println!("current: {}", output);
        }
    }

    unreachable!() */
    ".".to_string()
}
