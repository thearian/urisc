mod memory;
use memory::Memory;

mod assembler;
use assembler::compile_asm_to_bin;

mod simulator;
use simulator::simulate;

fn main() {
    let file = std::fs::read_to_string("test.g").unwrap();

    let bin = compile_asm_to_bin(file.clone());
    println!("FILE:\n{}\nBIN:\n{}",
        &file,
        bin.iter()
           .map(|num| { num.to_string() })
           .collect::<Vec<String>>()
           .join(" ")
    );
     let mut mem = Memory::new();
     mem.insert(bin);

     simulate(&mut mem);
}
