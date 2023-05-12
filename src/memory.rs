pub const MEMORY_SIZE: u8 = 64;

pub struct Memory {
    value: [i8; MEMORY_SIZE as usize]
}
impl Memory {
    pub fn new() -> Memory {
        Memory { value: [0;  MEMORY_SIZE as usize] }
    }
    pub fn get(&self, p: &Pointer) -> i8 {
        self.value[p.value as usize]
    }
    pub fn set(&mut self, p: &Pointer, value: i8) {
        if p.value < MEMORY_SIZE {
            self.value[p.value as usize] = value;
        }
        else {
            println!("memory warning: some SET was denied");
        }
    }
    pub fn display(&self) {
        let s = self.value.iter()
            .map(|num| { num.to_string() })
            .collect::<Vec<String>>()
            .join(" ");
        println!("mem {}", s);
    }
    pub fn next_segment(&self, index: u8) -> Option<[Pointer; 4]> {
        if index > (MEMORY_SIZE - 4) {
            return None;
        }
        else {
            let mut segment = [Pointer { value: 0 }; 4];
            for i in index..(index + 4) {
                segment[(i - index) as usize] = Pointer { value: i };
            }

            return Some(segment);
        }
    }
}

#[derive(Clone, Copy)]
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

