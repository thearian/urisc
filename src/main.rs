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
}

fn instruction(memory: &mut Memory, a: Pointer, b: Pointer, c: Pointer) -> Pointer {
    let ac = memory.get(&b) - memory.get(&a);
    memory.set(&b, ac);
    if ac >= 0 {
        return c
    }
    return c.next()
}
