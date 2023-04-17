use crate::memory::{Memory, Pointer, self};

pub type AssemblyItem = [u8; 3];
pub type Assembly = Vec<AssemblyItem>;

pub struct Binary {
    value: Vec<u8>
}
impl Binary {
    pub fn new() -> Binary {
        Binary { value: Vec::new() }
    }
    pub fn to_string(&self) -> String {
        self.value.iter()
            .map(|u| {
                format!("{}", u)
            })
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn append(&mut self, value: Vec<u8>) {
        self.value = [self.value.clone(), value].concat();
    }
}

pub fn link(assembly: Assembly) -> Binary {
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

pub fn run(bin: Binary) {
    let mut memory = Memory::new();
    for i in (0..bin.value.len()).step_by(3) {
        run_instruction(&mut memory,
            Pointer::new(bin.value[i]),
            Pointer::new(bin.value[i+1]),
            Pointer::new(bin.value[i+2]),
        );
        memory.display();
    }
}

fn run_instruction(memory: &mut Memory, a: Pointer, b: Pointer, c: Pointer) -> Pointer {
    let ac = memory.get(&b) - memory.get(&a);
    memory.set(&b, ac);
    if ac >= 0 {
        return c
    }
    return c.next()
}
