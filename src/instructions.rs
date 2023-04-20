use crate::{memory::Pointer, binary::Assembly};

pub enum Instruction {
 GOTO(Pointer),
 ADD(Pointer, Pointer),
 SET(Pointer, u8)
}

pub fn compile(code: String) -> Vec<Instruction> {
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
                },
                "set" => {
                    instructions.push(
                        Instruction::SET(
                            Pointer::new(
                                tokens[t+1].parse::<u8>()
                                    .expect("set arg is not number of pointer")
                            ),
                            tokens[t+2].parse::<u8>()
                                .expect("set arg is not value number")
                        )
                    );
                }
                _ => {}
            }
        }
    }

    return instructions;
}

pub fn assemble(instructions: Vec<Instruction>) -> Assembly {
    let mut assembly: Assembly = Vec::new();
    for ins in instructions {
        match ins {
            Instruction::GOTO(pointer) => {
                assembly.push(
                    [0, 0, 0, pointer.reference()]
                );
            }
            Instruction::ADD(a, b) => {
                assembly = [
                    assembly,
                    vec![
                        [0, a.reference(), 0, 0],
                        [0, 0,  b.reference(), 0],
                        [0, 0, 0, 0]
                    ]
                ].concat()
            }
            Instruction::SET(pointer, value) => {
                assembly.push(
                    [1, pointer.reference(), value, 0]
                );
            }
        };
    }
    return assembly;
}


