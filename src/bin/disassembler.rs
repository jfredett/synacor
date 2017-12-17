extern crate synacor;
extern crate clap;

use clap::{Arg, App};
use synacor::binary::Binary;

fn main() {
    let args = App::new("syn-dis")
        .version("v0.1.0")
        .author("Joe Fredette <jfredett.at.gmail.dot.com>")
        .about("Disassemble synacor binaries into raw assembly instructions")
        .arg(Arg::with_name("bin")
                 .short("b")
                 .long("bin")
                 .value_name("FILE")
                 .help("Path to the .bin to disassemble")
                 .takes_value(true))
        .arg(Arg::with_name("out")
                 .short("o")
                 .long("out")
                 .value_name("FILE")
                 .help("Path to dump the resulting assembly, defaults to the same name as input binary with .syn-asm extension"))
        .get_matches();


    let bin_path = String::from(args.value_of("bin").expect("Must provide ``--bin FILE''"));
    let mut b = Binary::new(&bin_path);

    println!("Parsing `{}'", bin_path);
    b.parse();

    //for instruction in b.instructions {
        //println!("{}", instruction);
    //}
}
