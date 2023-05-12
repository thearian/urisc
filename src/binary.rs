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
    pub fn write_to_memory(&self, memory: &mut Memory) {
        for (i, bit) in self.value.iter().enumerate() {
            memory.set(
                &Pointer::new(i as u8),
                *bit as i8
            );
        }
    }
    pub fn last_bit(&self) -> u8 {
        self.value.len() as u8
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
    bin.write_to_memory(&mut memory);

    let mut next_segment = memory.next_segment(0u8);
    memory.display();

    while let Some(segment) = next_segment {
        std::thread::sleep_ms(200);
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
    if ins.reference() == 0 {
        let ac = memory.get(b) - memory.get(a);
        memory.set(b, ac);
        if ac >= 0 {
            return Some(c.clone())
        }
        return c.next()
    } else {
        memory.set(a, b.reference() as i8);
        return c.next();
    }
}
