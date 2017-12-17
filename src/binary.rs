use std::fs::File;
use std::io::prelude::*;

use instruction::Instruction;
use argument::Argument;

#[derive(PartialEq, Eq, Debug)]
pub struct Binary {
    file: String,
    instructions: Vec<Instruction>,
    binary: Vec<u16>,
}

impl Binary {
    pub fn new(filepath: &String) -> Binary {
        Binary { file: filepath.to_owned(), instructions: vec![], binary: vec![] }
    }

    pub fn parse(&mut self) {
        let mut f = match File::open(&self.file) {
            Ok(file) => file,
            Err(error) => panic!("Could not open file: ``{}'', got error: ``{}''", self.file, error)
        };

        let mut v: Vec<u16> = vec![];
        let mut buf = [0u8; 2];
        let mut debug_idx = 0;
        loop {
            match f.read(&mut buf) {
                Err(_) => panic!("Error on reading byes during parse {:?}", buf),
                Ok(remaining) => {
                    if remaining == 0 { break ; }

                    let u : u16;
                    u = ((buf[1] as u16) << 8) | (buf[0] as u16);
                    v.push(u);
                    self.binary.push(u);
                },
            }
            debug_idx += 1;
        }


        while !v.is_empty() {
            let opcode = v.remove(1);
            let mut instruction = vec![opcode];

            let arg_count = match Instruction::arg_count(opcode) {
                Some(a) => a,
                None => break
            };

            for _ in 0..arg_count {
                instruction.push(v.remove(1));
            }

            // XXX: more hax
            self.instructions.push(Instruction::from_u16_sequence(&instruction).unwrap());
        }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn binary(&self) -> &Vec<u16> {
        &self.binary
    }
}
