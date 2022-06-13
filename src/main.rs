mod clock;
mod memory;

use crate::clock::Clock;
use crate::memory::Memory;
use std::time::Instant;

struct Registers {
    a: u8,
    y: u8,
    x: u8,
    pc: u16,
    s: u8,
    p: u8,
}

fn main() {
    println!("Hello, world!");

    let mut memory1 = Memory::new(None);
    let mut memory2 = Memory::new(Some([100; 65536]));

    assert_eq!(memory1.get(1), 0);
    memory1.put(1, 99);
    assert_eq!(memory1.get(1), 99);
    memory1.put_hl(1, 0, 99);
    assert_eq!(memory1.get_hl(1, 0), 99);
    assert_eq!(memory1.get(256), 99);
    
    assert_eq!(memory2.get(256), 100);
    memory2.put(1, 0);
    assert_eq!(memory2.get(1), 0);
    memory2.put_hl(1, 0, 0);
    assert_eq!(memory2.get_hl(1, 0), 0);
    assert_eq!(memory2.get(256), 0);
        
    let clock = Clock::new(2.0);
    let start = Instant::now();
    for tick in clock.wait {
        println!("tick: {:?}", tick.duration_since(start));
    }
}
