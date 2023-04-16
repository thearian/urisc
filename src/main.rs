mod memory;
use memory::{Memory, Pointer};

fn main() {
    let mut memory = Memory::new();

    let x = instruction(
        &mut memory,
        Pointer::new(0),
        Pointer::new(0),
        Pointer::new(3)
    );
    println!("{}",x.display());

    let asm: Assembly = vec![[0,0,3],[1,1,3]];
    let bin = link(asm);
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
