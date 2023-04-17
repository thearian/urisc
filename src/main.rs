mod memory;
use memory::{Memory, Pointer};

fn main() {
    let code = "add 1 2\ngoto 3".to_owned();
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
                },
                "add" => {
                    instructions.push(
                        Instruction::ADD(
                            Pointer::new(
                                tokens[t+1].parse::<u8>()
                                    .expect("add arg is not number of pointer")
                            ),
                            Pointer::new(
                                tokens[t+2].parse::<u8>()
                                    .expect("add arg is not number of pointer")
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
        match ins {
            Instruction::GOTO(pointer) => {
                assembly.push(
                    [0, 0, pointer.reference()]
                );
            }
            Instruction::ADD(a, b) => {
                assembly = [
                    assembly,
                    vec![
                        [a.reference(), 0, 0],
                        [0, b.reference(), 0],
                        [0, 0, 0]
                    ]
                ].concat()
            }
        };
    }
    return assembly;
}

enum Instruction {
 GOTO(Pointer),
 ADD(Pointer, Pointer)
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
