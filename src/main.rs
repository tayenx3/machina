mod vm;

use vm::MachinaArgon;
use vm::assembler;
use vm::registers::RDS;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 4 || args.len() < 2 {
        eprintln!("Usage: m0-32 <program>.mar32 (<output register>)");
        return;
    }

    let output = if args.len() == 3 {
        match assembler::parse_register(&args[2]) {
            Ok(reg) => reg,
            Err(err) => {
                eprintln!("{err}");
                return;
            },
        }
    } else {
        RDS
    };

    let mut vm = MachinaArgon::new();

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
