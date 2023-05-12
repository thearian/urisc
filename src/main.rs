mod memory;
use memory::{Memory, Pointer, ToUriscPointer};

fn test_move() {
    let mut mem = Memory::new();
    // DATA SEGMENT
    mem.set(&Pointer::new(1), 2i8);
    mem.set(&Pointer::new(2), 3i8);

    // CODE SEGMENT
    mem.set(&Pointer::new(3), 2i8);
    mem.set(&Pointer::new(4), 2i8);

    mem.set(&Pointer::new(5), 1i8);
    mem.set(&Pointer::new(6), 0i8);

    mem.set(&Pointer::new(7), 0i8);
    mem.set(&Pointer::new(8), 2i8);

    mem.set(&Pointer::new(9), 0i8);
    mem.set(&Pointer::new(10), 0i8);

    let mut ins = Pointer::new(3);
    for _ in 0..4 {
        mem.display();

        ins = subleq(&mut mem,
            &ins,
            &ins.next().unwrap(),
            &ins.next().unwrap().next().unwrap()
        ).unwrap();
    }
    mem.display();
}

fn test_add() {
    let mut mem = Memory::new();
    // DATA SEGMENT
    mem.set(&Pointer::new(1), 2i8);
    mem.set(&Pointer::new(2), 3i8);

    // CODE SEGMENT
    mem.set(&Pointer::new(3), 1i8);
    mem.set(&Pointer::new(4), 0i8);

    mem.set(&Pointer::new(5), 0i8);
    mem.set(&Pointer::new(6), 2i8);

    mem.set(&Pointer::new(7), 0i8);
    mem.set(&Pointer::new(8), 0i8);

    let mut ins = Pointer::new(3);
    for _ in 0..3 {
        mem.display();

        ins = subleq(&mut mem,
            &ins,
            &ins.next().unwrap(),
            &ins.next().unwrap().next().unwrap()
        ).unwrap();
    }
    mem.display();
}

fn subleq(
    mem: &mut Memory,
    a:   &    Pointer,
    b:   &    Pointer,
    c:   &    Pointer,
) -> Option<Pointer> {
    let ac = mem.get_value_of(b) - mem.get_value_of(a);
    mem.set_to_value_of(b, ac);
    if ac <= 0 {
        return Some(c.clone())
    }
    return b.next()
}
