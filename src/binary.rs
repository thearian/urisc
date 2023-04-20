use crate::memory::{Memory, Pointer};

pub type AssemblyItem = [u8; 4];
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
            ]
        )
    }

    return binary
}

pub fn run(bin: Binary) {
    let mut memory = Memory::new();
    let mut next_segment = memory.next_segment(0u8);

    while next_segment.is_some() {
        let segment = next_segment.unwrap();
        let next_pointer = run_instruction(
            &mut memory,
            &segment[0],
            &segment[1],
            &segment[2],
            &segment[3],
        );
        memory.display();
        next_segment = memory.next_segment(
            next_pointer.unwrap().reference()
        );
    }
}

fn run_instruction(
    memory: &mut Memory,
    ins: &Pointer, 
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
