mod memory;
use memory::{Memory, Pointer};

fn main() {
    let code = "goto 2".to_owned();
    let bin = link(
        assemble(
            compile(code)
        )
    );
    println!("{}", bin.to_string());
}

fn instruction(memory: &mut Memory, a: Pointer, b: Pointer, c: Pointer) -> Pointer {
    let ac = memory.get(&b) - memory.get(&a);
    memory.set(&b, ac);
    if ac >= 0 {
        return c
    }
    return c.next()
}

fn compile(code: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let lines = code.split('\n')
        .collect::<Vec<&str>>();

    for line in lines {
        let tokens = line.split(' ')
            .collect::<Vec<&str>>();

        for t in 0..tokens.len() {
            match tokens[t] {
                "goto" => {
                    instructions.push(
                        Instruction::GOTO(
                            Pointer::new(
                                tokens[t+1].parse::<u8>()
                                    .expect("goto arg is not number of pointer")
                            )
                        )
                    );
                }
                _ => {}
            }
        }
    }

    return instructions;
}

fn assemble(instructions: Vec<Instruction>) -> Assembly {
    let mut assembly: Assembly = Vec::new();
    for ins in instructions {
        let asm: AssemblyItem = match ins {
            Instruction::GOTO(pointer) => [0, 0, pointer.reference()]
        };
        assembly.push(asm);
    }
    return assembly;
}

enum Instruction {
 GOTO(Pointer)
}

fn link(assembly: Assembly) -> Binary {
    let mut binary = Binary::new();

    for assembly_item in assembly {
        binary.append(
            vec![
                assembly_item[0],
                assembly_item[1],
                assembly_item[2]
            ]
        )
    }

    return binary
}

type AssemblyItem = [u8; 3];
type Assembly = Vec<AssemblyItem>;

struct Binary {
    value: Vec<u8>
}
impl Binary {
    fn new() -> Binary {
        Binary { value: Vec::new() }
    }
    fn to_string(&self) -> String {
        self.value.iter()
            .map(|u| {
                format!("{}", u)
            })
            .collect::<Vec<String>>()
            .join("")
    }
    fn append(&mut self, value: Vec<u8>) {
        self.value = [self.value.clone(), value].concat();
    }
}
