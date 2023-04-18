use crate::memory::{Memory, Pointer};

pub type AssemblyItem = [u8; 5];
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
                assembly_item[2],
                assembly_item[3],
                assembly_item[4]
            ]
        )
    }

    return binary
}

pub fn run(bin: Binary) {
    let mut memory = Memory::new();
    let mut first_pointer = Some(
        Pointer::new(bin.value[0])
    );

    while first_pointer.is_some() {
        first_pointer = run_instruction(
            &mut memory,
            first_pointer.as_ref().unwrap(),
            first_pointer.as_ref().unwrap()
                .next().as_ref().unwrap(),
            first_pointer.as_ref().unwrap()
                .next().as_ref().unwrap()
                .next().as_ref().unwrap()
        );
        memory.display();
    }
}

fn run_instruction(
    memory: &mut Memory,
    a: &Pointer, 
    b: &Pointer,
    c: &Pointer
) -> Option<Pointer> {
    let ac = memory.get(b) - memory.get(a);
    memory.set(&b, ac);
    if ac >= 0 {
        return Some(c.clone())
    }
    return c.next()
}
