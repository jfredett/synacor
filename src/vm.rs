use std::convert::From;
use std::io;
use std::io::Read;

use u15::u15;
use address::Address;
use argument::Argument;
use register::Register;
use instruction::Instruction;
use constants::*;

pub struct VM {
    instruction_pointer: Address,
    stack: Vec<u16>,
    memory: [u16; U15_MAX as usize],
    registers: [u16; 8],
    current_state: VMState,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum VMState {
    RUN,
    HALT
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VMError {
    BadOpcode(u16),
    InvalidMemoryAccess(Address),
    MalformedInstruction(Vec<u16>),
    InvalidCharacterArgument(Argument),
    JumpOutOfBounds(Address),
    StackUnderflow,
    UnknownError
}

type VMResult = Result<VMState, VMError>;

impl VM {
    pub fn init() -> VM {
        VM {
            instruction_pointer: Address::new(0),
            stack: vec![],
            memory: [0; U15_MAX as usize],
            registers: [0; 8],
            current_state: VMState::HALT,
        }
    }

    pub fn instruction_pointer(&self) -> Address {
        return self.instruction_pointer;
    }

    /// Given an offset and some bytecode, write the bytecode to machine memory.
    pub fn load_program(&mut self, offset: Address, bytecode: &Vec<u16>) {
        let mut write_addr = offset;
        for v in bytecode {
            if write_addr.is_valid() {
                self.write_memory(&write_addr, *v);
                write_addr.next();
            } else {
                panic!("Attempted to load program, but ran out of memory.");
            }
        }
    }

    /// Given a series of raw instructions, compile them and write the resulting bytecode to memory
    pub fn load_instructions(&mut self, offset: Address, instructions: &Vec<Instruction>) {
        let mut program = vec![];
        for i in instructions {
            let mut bytecode = i.to_owned().to_u16_sequence();
            program.append(&mut bytecode);
        }
        self.load_program(offset, &program);
    }

    pub fn run(&mut self, start_position: Address) -> VMResult {
        self.instruction_pointer = start_position;
        self.current_state = VMState::RUN;

        while self.is_running() {
            match self.step() {
                Ok(state) => self.current_state = state,
                Err(e) => return Err(e)
            }
        }

        return Ok(self.current_state); // this should always end up being HALT here.
    }

    pub fn is_running(&self) -> bool {
        self.current_state == VMState::RUN
    }

    pub fn step(&mut self) -> VMResult {
        match self.current_instruction() {
            Ok(current_instruction) => self.execute_instruction(current_instruction),
            Err(e) => Err(e)
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> VMResult {
       match instruction {
           Instruction::HALT         => Ok(VMState::HALT),
           Instruction::SET(r,a)     => self.write_register(r, a),
           Instruction::PUSH(arg)    => { self.push(arg); Ok(VMState::RUN) } // push can never fail
           Instruction::POP(r)       => self.pop(r),
           Instruction::EQ(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               if a == b {
                   self.write_register(r, Argument::new(1))
               } else {
                   self.write_register(r, Argument::new(0))
               }
           },
           Instruction::GT(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               if a > b {
                   self.write_register(r, Argument::new(1))
               } else {
                   self.write_register(r, Argument::new(0))
               }
           },
           Instruction::JMP(a)       => self.jump(a),
           Instruction::JT(a,b)      => {
               if self.check_true(a) { return self.jump(b); }
               Ok(VMState::RUN)
           },
           Instruction::JF(a,b)      => {
               if !self.check_true(a) { return self.jump(b); }
               Ok(VMState::RUN)
           },
           Instruction::ADD(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               self.write_register(r, Argument::Literal(a + b))
           },
           Instruction::MULT(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               self.write_register(r, Argument::Literal(a * b))
           },
           Instruction::MOD(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               self.write_register(r, Argument::Literal(a % b))
           },
           Instruction::AND(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               self.write_register(r, Argument::Literal(a & b))
           },
           Instruction::OR(r, arg_a, arg_b)   => {
               let a : u15 = u15(self.parse_argument(arg_a));
               let b : u15 = u15(self.parse_argument(arg_b));

               self.write_register(r, Argument::Literal(a | b))
           },
           Instruction::NOT(r, arg_a)         => {
               let a : u15 = u15(self.parse_argument(arg_a));

               self.write_register(r, Argument::Literal(!a))
           },
           Instruction::RMEM(r,a)    => self.rmem(r,a),
           Instruction::WMEM(a,b)    => self.wmem(a,b),
           Instruction::CALL(a)      => self.call(a),
           Instruction::RET          => self.ret(),
           Instruction::OUT(a)       => self.write_output(a),
           Instruction::IN(a)        => self.read_input(a),
           Instruction::NOOP         => Ok(VMState::RUN),
       }
    }

    fn read_input(&mut self, a: Argument) -> VMResult {
        let mut stdin = io::stdin();
        let mut buf : [u8; 1] = [0; 1];

        stdin.read_exact(&mut buf);

        match a {
            Argument::Literal(addr) => {
                let target = Address::new(addr.0);
                self.write_memory(&target, buf[0] as u16);
                Ok(VMState::RUN)
            },
            Argument::Register(r) => {
                self.write_register(r, Argument::new(buf[0] as u16))
            }
        }
    }

    fn rmem(&mut self, r: Register, a: Argument) -> VMResult {
        let addr = Address::new(self.parse_argument(a));
        if let Ok(mem) = self.read_memory(&addr) {
            self.write_register(r, Argument::Literal(u15(mem)))
        } else {
            Err(VMError::InvalidMemoryAccess(addr))
        }
    }

    fn wmem(&mut self, t: Argument, v: Argument) -> VMResult {
        let target = Address::new(self.parse_argument(t));
        let value = self.parse_argument(v);

        self.write_memory(&target, value);

        Ok(VMState::RUN)
    }

    /// Push the address of the next instruction to the stack, jump to given address
    fn call(&mut self, a: Argument) -> VMResult {
        // get the position of the next instruction
        let cur_ptr = self.instruction_pointer.to_u16();
        self.push(Argument::new(cur_ptr));
        self.jump(a)
    }

    /// Pop the top of the stack, jump to the address attained.
    /// If empty, halt.
    ///
    /// Note that this is very similar to #pop, but does not error on StackUnderflow
    fn ret(&mut self) -> VMResult {
        match self.stack.pop() {
          Some(v) => self.jump(Argument::new(v)),
          None => Ok(VMState::HALT)
        }
    }

    /// Pop a value off the stack, put it in given register
    fn pop(&mut self, r: Register) -> VMResult {
        match self.stack.pop() {
          Some(v) => self.write_register(r, Argument::new(v)),
          None => Err(VMError::StackUnderflow)
        }
    }

    /// Push the given argument onto the stack
    fn push(&mut self, arg: Argument) {
        let v = self.parse_argument(arg);
        self.stack.push(v);
    }

    /// Checks if the argument is non-zero
    fn check_true(&self, arg: Argument) -> bool {
        let target = match arg {
            Argument::Literal(v) => v.0,
            Argument::Register(r) => self.read_register(r)
        };

        return target > 0;
    }

    /// Jump to the address given by the argument.
    fn jump(&mut self, arg: Argument) -> VMResult {
        let target = self.parse_argument(arg);

        let addr = Address::new(target);

        if addr.is_memory() {
            self.instruction_pointer = addr;
            return Ok(VMState::RUN);
        } else if addr.is_register() || addr.is_invalid() {
            return Err(VMError::JumpOutOfBounds(addr));
        } else {
            return Err(VMError::UnknownError);
        }
    }

    /// extract the value of an argument, either reading the register, or interpreting as a literal
    fn parse_argument(&self, arg: Argument) -> u16 {
        match arg {
            Argument::Literal(v) => v.0,
            Argument::Register(r) => self.read_register(r)
        }
    }

    /// writes the argument to stdout
    ///
    /// TODO: make this write to a buffer held in the VM struct
    fn write_output(&self, arg: Argument) -> VMResult {
        let chr = char::from(self.parse_argument(arg) as u8);

        if !chr.is_ascii() { return Err(VMError::InvalidCharacterArgument(arg)); }

        print!("{}", chr);

        Ok(VMState::RUN)
    }

    /// read the value stored in the given register
    fn read_register(&self, r: Register) -> u16 {
        return self.registers[r.as_index()];
    }

    /// write the given value to the given register
    fn write_register(&mut self, r: Register, a: Argument) -> VMResult {
        let arg = self.parse_argument(a);

        self.registers[r.as_index()] = arg;

        Ok(VMState::RUN)
    }

    /// write the given value at the given address in memory.
    fn write_memory(&mut self, address: &Address, value: u16) {
        self.memory[address.value() as usize] = value;
    }

    /// Read the value at memory address `location`
    fn read_memory(&self, location: &Address) -> Result<u16, VMError> {
        if location.is_invalid() { return Err(VMError::InvalidMemoryAccess(*location)); }
        Ok(self.memory[location.to_usize()])
    }

    fn current_instruction(&mut self) -> Result<Instruction, VMError> {
        let opcode = match self.advance() {
            Ok(o) => o,
            Err(e) => return Err(e)
        };

        let arg_count = match Instruction::arg_count(opcode) {
            Some(a) => a,
            None => return Err(VMError::BadOpcode(opcode))
        };

        let mut opcode_sequence = vec![opcode];
        for _ in 0..arg_count {
            match self.advance() {
                Ok(arg) => opcode_sequence.push(arg),
                Err(err) => return Err(err)
            }
        }

        match Instruction::from_u16_sequence(&opcode_sequence) {
            Some(i) => Ok(i),
            None => Err(VMError::MalformedInstruction(opcode_sequence))
        }
    }

    /// Get the current value at the instruction_pointer and advance the pointer forward
    /// one address.
    fn advance(&mut self) -> Result<u16, VMError> {
        let ret = self.read_memory(&self.instruction_pointer);
        self.instruction_pointer.next();
        return ret
    }
}


#[cfg(test)]
mod tests {
    use register::Register;

    fn example_program() -> Vec<u16> {
        // FROM THE SPEC:
        //- The program "9,32768,32769,4,19,32768" occupies six memory addresses and should:
        //  - Store into register 0 the sum of 4 and the value contained in register 1.
        //  - Output to the terminal the character with the ascii code contained in register 0.
        vec![9,32768,32769,4,19,32768]
    }

    // loaded vm with the example program
    fn loaded_vm() -> VM {
        let mut vm = VM::init();
        vm.load_program(Address::new(1000), &example_program());
        return vm;
    }

    use super::*;

    mod load_program {
        use super::*;

        #[test]
        fn valid_program_load() {
            let mut vm = VM::init();
            vm.load_program(Address::new(1000), &example_program());

            assert_eq!(vm.memory[1000], 9);
            assert_eq!(vm.memory[1001], 32768);
            assert_eq!(vm.memory[1002], 32769);
            assert_eq!(vm.memory[1003], 4);
            assert_eq!(vm.memory[1004], 19);
            assert_eq!(vm.memory[1005], 32768);
        }
    }

    mod instructions {
        use super::*;

        mod gt {
            use super::*;

            #[test]
            fn lit_lit_false() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::GT(Register::R0, Argument::new(2), Argument::new(2))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }

            #[test]
            fn lit_lit_true() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::GT(Register::R0, Argument::new(3), Argument::new(2))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn lit_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(13)),
                    Instruction::GT(Register::R0, Argument::new(15), Argument::new(REGISTER_1))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(13)),
                    Instruction::GT(Register::R0, Argument::new(REGISTER_1), Argument::new(15))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(2)),
                    Instruction::SET(Register::R0, Argument::new(1)),
                    Instruction::GT(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }
        }

        mod eq {
            use super::*;

            #[test]
            fn lit_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::EQ(Register::R0, Argument::new(2), Argument::new(2))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn lit_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::EQ(Register::R0, Argument::new(15), Argument::new(REGISTER_1))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::EQ(Register::R0, Argument::new(REGISTER_1), Argument::new(15))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(1)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::EQ(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }
        }

        mod not {
            use super::*;

            #[test]
            fn lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::NOT(Register::R0, Argument::new(4))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(u15(vm.registers[0]), !u15(4));
            }


            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::NOT(Register::R0,  Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(u15(vm.registers[0]), !u15(15));
            }
        }

        mod or {
            use super::*;

            #[test]
            fn lit_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::OR(Register::R0, Argument::new(2), Argument::new(4))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 4 | 2);
            }


            #[test]
            fn lit_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::OR(Register::R0, Argument::new(4), Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15 | 4);
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(10)),
                    Instruction::OR(Register::R0, Argument::new(REGISTER_1), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 10 | 2);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(10)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::OR(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 10 | 2);
            }
        }

        mod and {
            use super::*;

            #[test]
            fn lit_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::OR(Register::R0, Argument::new(2), Argument::new(5))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 2 | 5);
            }


            #[test]
            fn lit_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::AND(Register::R0, Argument::new(4), Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15 & 4);
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::AND(Register::R0, Argument::new(REGISTER_1), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15 & 2);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::AND(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15 & 2);
            }
        }

        mod add {
            use super::*;

            #[test]
            fn lit_lit_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::ADD(Register::R0, Argument::new(2), Argument::new(2))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 4);
            }

            #[test]
            fn lit_lit_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::ADD(Register::R0, Argument::new(3), Argument::new(MODULUS - 3))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }

            #[test]
            fn lit_reg_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::ADD(Register::R0, Argument::new(2), Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 17);
            }

            #[test]
            fn lit_reg_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-2)),
                    Instruction::ADD(Register::R0, Argument::new(2), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }

            #[test]
            fn reg_lit_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::ADD(Register::R0, Argument::new(REGISTER_1), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 17);
            }

            #[test]
            fn reg_lit_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-2)),
                    Instruction::ADD(Register::R0, Argument::new(REGISTER_0), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }

            #[test]
            fn reg_reg_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::ADD(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 17);
            }

            #[test]
            fn reg_reg_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-2)),
                    Instruction::SET(Register::R1, Argument::new(2)),
                    Instruction::ADD(Register::R0, Argument::new(REGISTER_0), Argument::new(REGISTER_1)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 0);
            }
        }

        mod mult {
            use super::*;

            #[test]
            fn lit_lit_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::MULT(Register::R0, Argument::new(2), Argument::new(2))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 4);
            }

            #[test]
            fn lit_lit_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::MULT(Register::R0, Argument::new(2), Argument::new(MODULUS - 1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], MODULUS - 2);
            }

            #[test]
            fn lit_reg_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::MULT(Register::R0, Argument::new(2), Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 30);
            }

            #[test]
            fn lit_reg_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-1)),
                    Instruction::MULT(Register::R0, Argument::new(2), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], MODULUS - 2);
            }

            #[test]
            fn reg_lit_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::MULT(Register::R0, Argument::new(REGISTER_1), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 30);
            }

            #[test]
            fn reg_lit_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-1)),
                    Instruction::MULT(Register::R0, Argument::new(REGISTER_0), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], MODULUS - 2);
            }

            #[test]
            fn reg_reg_nowrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::MULT(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 30);
            }

            #[test]
            fn reg_reg_wrap() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(MODULUS-2)),
                    Instruction::SET(Register::R1, Argument::new(2)),
                    Instruction::MULT(Register::R0, Argument::new(REGISTER_0), Argument::new(REGISTER_1)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], MODULUS - 4);
            }
        }

        mod modulo {
            use super::*;

            #[test]
            fn lit_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::MOD(Register::R0, Argument::new(5), Argument::new(2))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn lit_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                        Instruction::SET(Register::R1, Argument::new(15)),
                        Instruction::MOD(Register::R0, Argument::new(20), Argument::new(REGISTER_1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 5);
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::MOD(Register::R0, Argument::new(REGISTER_1), Argument::new(2)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R1, Argument::new(15)),
                    Instruction::SET(Register::R0, Argument::new(2)),
                    Instruction::MOD(Register::R0, Argument::new(REGISTER_1), Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 1);
            }
        }

        mod set {
            use super::*;


            #[test]
            fn lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(15)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15);
            }

            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(15)),
                    Instruction::SET(Register::R1, Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15);
                assert_eq!(vm.registers[1], 15);
            }
        }

        mod jump {
            use super::*;

            #[test]
            fn lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![Instruction::JMP(Argument::new(10))]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                // it goes to @11 because it has to read the halt instruction at @10.
                assert_eq!(vm.instruction_pointer, Address::new(11));
            }

            // OOB isn't possible due to types, the instruction would fail to parse and we'd fail
            // further up. We might run into it with registers though, since SET isn't implemented


            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(15)),
                    Instruction::JMP(Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                // it goes to @16 because it has to read the halt instruction at @15.
                assert_eq!(vm.instruction_pointer, Address::new(16));
            }
        }

        mod jt {
            use super::*;

            // why you would ever need this is beyond me, maybe for self modifying code?
            #[test]
            fn lit_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::JT(Argument::new(1), Argument::new(10))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::EQ(Register::R0, Argument::new(2), Argument::new(2)),
                    Instruction::JT(Argument::new(REGISTER_0), Argument::new(10))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }

            // same as lit_lit, why?
            #[test]
            fn lit_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R6, Argument::new(10)),
                    Instruction::JT(Argument::new(0), Argument::new(REGISTER_6))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(7));
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::EQ(Register::R0, Argument::new(2), Argument::new(2)),
                    Instruction::SET(Register::R6, Argument::new(10)),
                    Instruction::JT(Argument::new(REGISTER_0), Argument::new(REGISTER_6))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }
        }

        mod jf {
            use super::*;

            // why you would ever need this is beyond me, maybe for self modifying code?
            #[test]
            fn lit_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::JF(Argument::new(0), Argument::new(10))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }

            #[test]
            fn reg_lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::EQ(Register::R0, Argument::new(3), Argument::new(2)),
                    Instruction::JF(Argument::new(REGISTER_0), Argument::new(10))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }

            // same as lit_lit, why?
            #[test]
            fn lit_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R6, Argument::new(10)),
                    Instruction::JF(Argument::new(10), Argument::new(REGISTER_6))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(7));
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::EQ(Register::R0, Argument::new(3), Argument::new(2)),
                    Instruction::SET(Register::R6, Argument::new(10)),
                    Instruction::JF(Argument::new(REGISTER_0), Argument::new(REGISTER_6))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
            }
        }

        mod push {
            use super::*;

            #[test]
            fn lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::PUSH(Argument::new(10))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.stack[0], 10);
            }

            #[test]
            fn reg() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(3)),
                    Instruction::PUSH(Argument::new(REGISTER_0))
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.stack[0], 3);
            }
        }

        mod pop {
            use super::*;

            #[test]
            fn happy() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::PUSH(Argument::new(10)),
                    Instruction::POP(Register::R0)
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 10);
            }

            #[test]
            fn nonempty_remaining_stack() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::PUSH(Argument::new(1)),
                    Instruction::PUSH(Argument::new(2)),
                    Instruction::POP(Register::R0)
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 2);
                assert!(vm.stack.len() == 1);
            }

            #[test]
            fn stack_underflow_error() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::POP(Register::R0)
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Err(VMError::StackUnderflow));

            }
        }

        mod rmem {
            use super::*;


            #[test]
            fn lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::RMEM(Register::R0, Argument::new(0)),
                    Instruction::RMEM(Register::R1, Argument::new(1))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[0], 15);
                assert_eq!(vm.registers[1], REGISTER_0);
            }

            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(1)),
                    Instruction::RMEM(Register::R1, Argument::new(REGISTER_0))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[1], REGISTER_0);
            }
        }

        mod wmem {
            use super::*;

            #[test]
            fn lit_lit() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::WMEM(Argument::new(1000), Argument::new(15)),
                    Instruction::RMEM(Register::R1, Argument::new(1000))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[1], 15);
            }

            #[test]
            fn lit_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(17)),
                    Instruction::WMEM(Argument::new(1000), Argument::new(REGISTER_0)),
                    Instruction::RMEM(Register::R1, Argument::new(1000))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[1], 17);
            }

            #[test]
            fn reg_reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(1000)),
                    Instruction::SET(Register::R1, Argument::new(18)),
                    Instruction::WMEM(Argument::new(REGISTER_0), Argument::new(REGISTER_1)),
                    Instruction::RMEM(Register::R1, Argument::new(REGISTER_0))
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.registers[1], 18);
            }
        }

        mod call {
            use super::*;

            #[test]
            fn lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![Instruction::CALL(Argument::new(10))]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(11));
                assert_eq!(vm.stack[0], 2);
            }

            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(15)),
                    Instruction::CALL(Argument::new(REGISTER_0)) 
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(16));
                assert_eq!(vm.stack[0], 5);
            }
        }

        mod ret {
            use super::*;

            #[test]
            fn lit() {
                let mut vm = VM::init();

                vm.load_instructions(Address::new(0), &vec![
                    Instruction::CALL(Argument::new(5)),
                    Instruction::HALT,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::NOOP,
                    Instruction::RET
                ]);

                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(3));
                assert!(vm.stack.is_empty());
            }

            #[test]
            fn reg() {
                let mut vm = VM::init();
                vm.load_instructions(Address::new(0), &vec![
                    Instruction::SET(Register::R0, Argument::new(6)), // 3 => @2
                    Instruction::CALL(Argument::new(REGISTER_0)),     // 2 => @4
                    Instruction::HALT, // @5
                    Instruction::NOOP, // @6
                    Instruction::RET   // @7
                ]);
                let result = vm.run(Address::new(0));
                assert_eq!(result, Ok(VMState::HALT));

                assert_eq!(vm.instruction_pointer, Address::new(6));
                assert!(vm.stack.is_empty());
            }
        }
    }

    mod step {
        use super::*;

        #[test]
        fn step() {
            let mut vm = loaded_vm();

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1000);

            assert!(vm.stack.is_empty());
            assert_eq!(vm.registers[0], 0);
            assert_eq!(vm.registers[1], 0);

            let mut result = vm.step();

            assert_eq!(result, Ok(VMState::RUN));

            assert!(vm.stack.is_empty());
            assert_eq!(vm.registers[0], 4);
            assert_eq!(vm.registers[1], 0);

            result = vm.step();
            // FIXME: this should output the ascii value '4' to an output stream, since I don't
            // have the output stream injected yet, no good way to test for that.
            assert_eq!(result, Ok(VMState::RUN));

            assert_eq!(vm.instruction_pointer, Address::new(1006));

            result = vm.step();
            assert_eq!(result, Ok(VMState::HALT));
            assert_eq!(vm.instruction_pointer, Address::new(1007));
        }

        #[test]
        fn advance() {
            let mut vm = loaded_vm();

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1000);
            let val = vm.advance();

            assert_eq!(vm.instruction_pointer, Address::new(1001));
            assert_eq!(val, Ok(9));
        }

        #[test]
        fn read_memory_happy() {
            let vm = loaded_vm();
            let ptr = Address::new(1000);
            assert_eq!(vm.read_memory(&ptr), Ok(9));
        }

        #[test]
        fn read_memory_invalid() {
            let vm = loaded_vm();
            let ptr = Address::new(40000);
            assert_eq!(vm.read_memory(&ptr), Err(VMError::InvalidMemoryAccess(ptr)));
        }

        #[test]
        fn current_instruction_happy() {
            let mut vm = loaded_vm();

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1000);
            assert_eq!(
                vm.current_instruction(),
                Ok(Instruction::ADD(Register::R0, Argument::new(REGISTER_1), Argument::new(4)))
            );
            assert_eq!(vm.instruction_pointer, Address::new(1004));
        }

        #[test]
        fn current_instruction_bad_opcode() {
            let mut vm = loaded_vm();

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1001);
            assert_eq!(
                vm.current_instruction(),
                Err(VMError::BadOpcode(REGISTER_0))
            );
            assert_eq!(vm.instruction_pointer, Address::new(1002));
        }

        #[test]
        #[should_panic] // XXX: It really shouldn't, but I don't want to refactor this right now.
        fn current_instruction_malformed() {
            let mut vm = loaded_vm();

            vm.write_memory(&Address::new(1002), 40000); // write some bad value into memory, out of bounds or w/e

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1000);
            assert_eq!(
                vm.current_instruction(),
                Err(VMError::MalformedInstruction(vec![9,REGISTER_0, 40000]))
            );
            assert_eq!(vm.instruction_pointer, Address::new(1004));
        }
    }
}
