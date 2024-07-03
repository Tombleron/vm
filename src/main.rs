mod assembly;
mod instruction;
mod repl;
mod vm;

fn main() {
    let program = assembly::Program::from_file("program.asm").unwrap();
    let mut vm = vm::Vm::new();
    vm.program = program.to_bytes();
    vm.run();
    println!("{:#?}", vm);
}
