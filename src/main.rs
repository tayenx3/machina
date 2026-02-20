mod vm;

use vm::M0_32;
use vm::assembler;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: m0-32 <program>.m0asm <output register>")
    }

    let output = match assembler::parse_register(&args[2]) {
        Ok(reg) => reg,
        Err(err) => {
            eprintln!("{err}");
            return;
        },
    };

    let mut vm = M0_32::new();

    match assembler::assemble_from_path(&args[1]) {
        Ok(path) => if let Err(err) = vm.load_program_from_path(path) {
            eprintln!("{err}");
            return;
        },
        Err(err) => {
            eprintln!("{err}");
            return;
        },
    }
    
    vm.run();
    println!("{}", vm.registers[output as usize]);
}
