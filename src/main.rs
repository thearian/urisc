mod memory;
use memory::{Memory, Pointer, ToUriscPointer};

fn main() {

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
    let bb = mem.get(&mem.get(&b).to_urisc_pointer());
    let aa = mem.get(&mem.get(&a).to_urisc_pointer());

    let ac = bb - aa;
    mem.set(&mem.get(&b).to_urisc_pointer(), ac);
    // mem.set(&b, ac);
    println!("{} {} {}, {}-{}={}",a.display(),b.display(),c.display()
        , bb, aa
        ,ac);
    if ac <= 0 {
        return Some(c.clone())
    }
    return b.next()
}
