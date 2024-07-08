use crate::instruction::Opcode;

const REGISTER_COUNT: usize = 32;

#[derive(Debug)]
pub struct Vm {
    pub registers: [i32; REGISTER_COUNT],
    pub pc: usize,
    pub program: Vec<u8>,
    pub heap: Vec<u8>,

    pub rem: u32,
    pub cmp: u32,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            registers: [0; REGISTER_COUNT],
            pc: 0,
            program: vec![],
            heap: vec![],
            rem: 0,
            cmp: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                return true;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as i32;
                self.registers[register] = number;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.rem = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 == register2) as u32;
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 != register2) as u32;
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 > register2) as u32;
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 < register2) as u32;
            }
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 >= register2) as u32;
            }
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.cmp = (register1 <= register2) as u32;
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.cmp == 1 {
                    self.pc = target as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.cmp == 0 {
                    self.pc = target as usize;
                }
            }
            Opcode::ALLOC => {
                let size = self.registers[self.next_8_bits() as usize];
                let new_heap_len = self.heap.len() + size as usize;
                self.heap.resize(new_heap_len, 0);
            }
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            }
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] -= 1;
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }

        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = Vm::new();
        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.registers[REGISTER_COUNT - 1], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = Vm::new();
        vm.program = vec![0, 0, 0, 0];
        vm.run();

        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = Vm::new();
        vm.program = vec![255, 0, 0, 0];
        vm.run();

        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = Vm::new();
        vm.program = vec![1, 0, 1, 244];
        vm.run_once();

        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 250, // LOAD 250 to register 0
            1, 1, 0, 250, // LOAD 250 to register 1
            2, 0, 1, 2,
        ]; // ADD register 0 and register 1, store result in register 2
        vm.run();

        assert_eq!(vm.registers[0], 250);
        assert_eq!(vm.registers[1], 250);
        assert_eq!(vm.registers[2], 500);
    }

    #[test]
    fn test_opcode_sub() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 1, 244, // LOAD 500 to register 0
            1, 1, 0, 250, // LOAD 250 to register 1
            3, 0, 1, 2, // SUB register 0 and register 1, store result in register 2
        ];
        vm.run();

        assert_eq!(vm.registers[0], 500);
        assert_eq!(vm.registers[1], 250);
        assert_eq!(vm.registers[2], 250);
    }

    #[test]
    fn test_opcode_mul() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 2 to register 0
            1, 1, 0, 3, // LOAD 3 to register 1
            4, 0, 1, 2, // MUL register 0 and register 1, store result in register 2
        ];
        vm.run();

        assert_eq!(vm.registers[0], 2);
        assert_eq!(vm.registers[1], 3);
        assert_eq!(vm.registers[2], 6);
    }

    #[test]
    fn test_opcode_div() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 11, // LOAD 10 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            5, 0, 1, 2, // DIV register 0 and register 1, store result in register 2
        ];
        vm.run();

        assert_eq!(vm.registers[0], 11);
        assert_eq!(vm.registers[1], 2);
        assert_eq!(vm.registers[2], 5);
        assert_eq!(vm.rem, 1);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 5 to register 0
            6, 0, // JMP to register 0
        ];
        vm.run_once();
        assert_eq!(vm.registers[0], 1);
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 1 to register 0
            8, 0, // JMPB to register 0
        ];
        vm.run_once();
        assert_eq!(vm.pc, 4);
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 4, // LOAD 1 to register 0
            7, 0, // JMPF to register 0
            1, 0, 0, 1, // LOAD 1 to register 0
        ];
        vm.run_once();
        assert_eq!(vm.pc, 4);
        vm.run_once();
        assert_eq!(vm.pc, 10);
    }

    #[test]
    fn test_opcode_eq() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            9, 0, 1, // EQ register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            9, 0, 1, // EQ register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);
    }

    #[test]
    fn test_opcode_neq() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            10, 0, 1, // NEQ register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            10, 0, 1, // NEQ register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);
    }

    #[test]
    fn test_opcode_gt() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 2 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            11, 0, 1, // GT register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            11, 0, 1, // GT register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);
    }

    #[test]
    fn test_opcode_lt() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            12, 0, 1, // LT register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 2 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            12, 0, 1, // LT register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);
    }

    #[test]
    fn test_opcode_gte() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 2 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            13, 0, 1, // GTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            13, 0, 1, // GTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            13, 0, 1, // GTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);
    }

    #[test]
    fn test_opcode_lte() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 2, // LOAD 2 to register 1
            14, 0, 1, // LTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 2, // LOAD 2 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            14, 0, 1, // LTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 0);

        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 1, // LOAD 1 to register 0
            1, 1, 0, 1, // LOAD 1 to register 1
            14, 0, 1, // LTE register 0 and register 1, store result in register 2
        ];
        vm.run();
        assert_eq!(vm.cmp, 1);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut vm = Vm::new();
        vm.program = vec![1, 0, 0, 17, 1, 1, 0, 17, 9, 0, 1, 15, 1, 1, 0, 0, 2];
        vm.run();
        assert_eq!(vm.pc, 17);
        assert_eq!(vm.registers[0], 17);

        let mut vm = Vm::new();
        vm.program = vec![1, 0, 0, 17, 1, 1, 0, 18, 9, 0, 1, 15, 1, 1, 0, 0, 2];
        vm.run();
        assert_eq!(vm.pc, 17);
        assert_eq!(vm.registers[0], 2);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut vm = Vm::new();
        vm.program = vec![1, 0, 0, 17, 1, 1, 0, 17, 10, 0, 1, 16, 1, 1, 0, 0, 2];
        vm.run();
        assert_eq!(vm.pc, 17);
        assert_eq!(vm.registers[0], 17);

        let mut vm = Vm::new();
        vm.program = vec![1, 0, 0, 17, 1, 1, 0, 18, 10, 0, 1, 16, 1, 1, 0, 0, 2];
        vm.run();
        assert_eq!(vm.pc, 17);
        assert_eq!(vm.registers[0], 2);
    }

    #[test]
    fn test_opcode_alloc() {
        let mut vm = Vm::new();
        vm.program = vec![
            1, 0, 0, 10, // LOAD 10 to register 0
            17, 0,
        ];
        vm.run();
        assert_eq!(vm.heap.len(), 10);
    }
}
