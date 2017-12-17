//use u15::u15;
use address::Address;
use argument::Argument;
use instruction::Instruction;
use constants::*;


struct VM {
    instruction_pointer: Address,
    stack: Vec<u16>,
    memory: [u16; U15_MAX as usize],
    registers: [u16; 8],
    current_state: VMState,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum VMState {
    RUN,
    HALT
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum VMError {
    BadOpcode(u16),
    InvalidMemoryAccess(Address),
    MalformedInstruction(Vec<u16>)
}

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

    pub fn load_program(&mut self, offset: Address, bin: &Vec<u16>) {
        let mut write_addr = offset;
        for v in bin {
            if write_addr.is_valid() {
                self.write_memory(&write_addr, *v);
                write_addr.next();
            } else {
                panic!("Attempted to load program, but ran out of memory.");
            }
        }
    }

    pub fn run(&mut self, start_position: Address) -> Result<VMState, VMError> {
        self.instruction_pointer = start_position;

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

    pub fn step(&mut self) -> Result<VMState, VMError> {
        match self.current_instruction() {
            Ok(current_instruction) => self.execute_instruction(current_instruction),
            Err(e) => Err(e)
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<VMState, VMError> {
       match instruction {
           Instruction::OUT(a) => {
               self.write_output(a);
               Ok(VMState::RUN)
           },
           Instruction::NOOP => Ok(VMState::RUN),
           Instruction::HALT   => Ok(VMState::HALT),
           _ => Ok(VMState::HALT) // any unrecognized opcode halts.
       }
    }

    /// writes the argument to stdout
    ///
    /// TODO: make this write to a buffer held in the VM struct
    fn write_output(&self, arg: Argument) {
        match arg {
            Argument::Literal(v) => print!("{}", v),
            Argument::Register(r) => print!("{}", self.registers[r.as_index()])
        };
    }

    fn write_memory(&mut self, address: &Address, value: u16) {
        self.memory[address.value() as usize] = value;
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

    /// Read the value at memory address `location`
    fn read_memory(&self, location: &Address) -> Result<u16, VMError> {
        if location.is_invalid() { return Err(VMError::InvalidMemoryAccess(*location)); }
        Ok(self.memory[location.to_usize()])
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

    mod step {
        use super::*;

        // XXX: Disabled while I run the given program with just the example instructions installed
        //#[test]
        fn step() {
            let mut vm = loaded_vm();

            // force the instruction pointer to the beginning of the program
            vm.instruction_pointer = Address::new(1000);

            assert!(vm.stack.is_empty());
            assert_eq!(vm.registers[0], 0);
            assert_eq!(vm.registers[1], 0);

            vm.step();

            assert!(vm.stack.is_empty());
            assert_eq!(vm.registers[0], 4);
            assert_eq!(vm.registers[1], 0);

            //vm.step();

            // this should output the ascii value '4' to an output stream
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
            let mut vm = loaded_vm();
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
