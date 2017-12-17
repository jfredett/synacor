use std::fs::File;
use std::io::prelude::*;

use instruction::Instruction;

pub struct Binary {
    file: String,
    instructions: Vec<Instruction>,
}

impl Binary {
    pub fn new(filepath: &String) -> Binary {
        Binary { file: filepath.to_owned(), instructions: vec![] }
    }

    pub fn parse(&mut self) {
        let mut f = match File::open(&self.file) {
            Ok(file) => file,
            Err(error) => panic!("Could not open file: ``{}'', got error: ``{}''", self.file, error)
        };

        let mut v: Vec<u16> = vec![];
        let mut buf = [0u8; 2];
        loop {
            match f.read(&mut buf) {
                Err(_) => break, // lop off any trailing bytes
                Ok(_) => (), // we'll use the buffer below
            }

            let u : u16;
            u = ((buf[0] as u16) << 15) | (buf[1] as u16);
            v.push(u);
        }

        while !v.is_empty() {
            let opcode = v.remove(1);
            let mut instruction = vec![opcode];
            let arg_count = Instruction::arg_count(opcode);

            for _ in 0..arg_count {
                instruction.push(v.remove(1));
            }

            self.instructions.push(Instruction::from_u16_sequence(&instruction));
        }
    }
}
