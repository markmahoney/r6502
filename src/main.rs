pub mod address_decoder;
pub mod clock;
pub mod io_device;
pub mod memory;

use crate::clock::Clock;
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;
use crate::address_decoder::AddressDecoder;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut decoder = AddressDecoder::new();
    decoder.add_device(
        0x0000..=0x7FFF,
        Box::new(RAM::<0x8000>::new(None)),
    );
    decoder.add_device(
        0x8000..=0xFFFF,
        Box::new(ROM::<0x8000>::new(Some([255; 0x8000]))),
    );
    
    let clock = Clock::new(2.0);
    let start = Instant::now();
    for tick in clock.wait {
        println!("tick: {:?}", tick.duration_since(start));
    }
}
