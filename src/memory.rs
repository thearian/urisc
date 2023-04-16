pub const MEMORY_SIZE: i8 = 4;

pub struct Memory {
    value: [i8; MEMORY_SIZE as usize]
}
impl Memory {
    pub fn new() -> Memory {
        Memory { value: [0,0,0,0] }
    }
    pub fn get(&self, p: &Pointer) -> i8 {
        self.value[p.value as usize]
    }
    pub fn set(&mut self, p: &Pointer, value: i8) {
        self.value[p.value as usize] = value;
    }
}

pub struct Pointer {
    value: i8
}
impl Pointer {
    pub fn new(value: i8) -> Pointer {
        if value > MEMORY_SIZE {
            panic!("Memory overflow: attempt to create pointer of {}", value)
        }
        return Pointer { value }
    }
    pub fn next(&self) -> Pointer {
        Pointer { value: self.value + 1 }
    }
    pub fn display(&self) -> String {
        format!("ptr {}", self.value)
    } 
}

