use crate::memory::{Memory, Pointer, ToUriscPointer};

pub fn simulate(mem: &mut Memory) {
    let mut ins = Pointer::new(1);
    for _ in 0..7 {
        mem.display();

        ins = subleq(mem,
            &ins,
            &ins.next().unwrap(),
            &ins.next().unwrap().next().unwrap()
        ).unwrap();
    }
    mem.display();
}

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
    // println!("{} {} {}, {}-{}={}", a.display(), b.display(), c.display(),
        // mem.get_value_of(b), mem.get_value_of(a), mem.get_value_of(b) - mem.get_value_of(a));

    let ac = mem.get_value_of(b) - mem.get_value_of(a);
    mem.set_to_value_of(b, ac);

    if ac <= 0 {
         let goto = mem.get_value_of(
             &mem.get_value_of(c).to_urisc_pointer()
         ).to_urisc_pointer();

        // println!(" goto {}", goto.display());
        return Some(goto)
    }
    return b.next()
}
