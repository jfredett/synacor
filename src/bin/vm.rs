extern crate synacor;
extern crate clap;

use clap::{Arg, App};
use synacor::binary::Binary;
use synacor::vm::VM;
use synacor::address::Address;

use std::io::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp;

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.trim().parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}


fn main() {
    let args = App::new("syn-vm")
        .version("v0.1.0")
        .author("Joe Fredette <jfredett.at.gmail.dot.com>")
        .about("Run programs on the synacor vm")
        .arg(Arg::with_name("bin")
                 .short("b")
                 .long("bin")
                 .value_name("FILE")
                 .help("Path to the .bin to run")
                 .takes_value(true))
        .arg(Arg::with_name("offset")
                 .short("o")
                 .long("offset")
                 .help("Where to start the program")
                 .takes_value(true))
        .get_matches();


    let bin_path = String::from(args.value_of("bin").expect("Must provide ``--bin FILE''"));
    let offset = parse_as::<u16>(&String::from(args.value_of("offset").unwrap_or("0")));
    let mut b = Binary::new(&bin_path);

    println!("Parsing `{}'", bin_path);
    b.parse();

    println!("Initializing VM");
    let mut vm = VM::init();

    println!("Loading Program: `{}'", bin_path);
    vm.load_program(Address::new(0), b.binary());

    println!("Running...");
    println!("");

    match vm.run(Address::new(offset)) {
        Ok(state) => println!("SUCCESS: Program Finished with: {:?}", state),
        Err(e) => println!("ERROR: Program Finished with: {:?}", e),
    }

    println!("");
    println!("Ended on instruction: {}", vm.instruction_pointer());
}

