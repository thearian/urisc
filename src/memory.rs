pub const MEMORY_SIZE: u8 = 16;

pub struct Memory {
    value: [i8; MEMORY_SIZE as usize]
}
impl Memory {
    pub fn new() -> Memory {
        Memory { value: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] }
    }
    pub fn get(&self, p: &Pointer) -> i8 {
        self.value[p.value as usize]
    }
    pub fn set(&mut self, p: &Pointer, value: i8) {
        self.value[p.value as usize] = value;
    }
    pub fn display(&self) {
        println!("{:?}", self.value)
    }
}

#[derive(Clone)]
pub struct Pointer {
    value: u8
}
impl Pointer {
    pub fn new(value: u8) -> Pointer {
        if value > MEMORY_SIZE {
            panic!("Memory overflow: attempt to create pointer of {}", value)
        }
        return Pointer { value }
    }
    pub fn next(&self) -> Option<Pointer> {
        if self.value < MEMORY_SIZE {
            Some(Pointer { value: self.value + 1 })
        } else {
            None
        }
    }
    pub fn display(&self) -> String {
        format!("ptr {}", self.value)
    } 
    pub fn reference(&self) -> u8 {
        self.value
    }
}

