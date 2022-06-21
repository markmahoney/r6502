mod clock;
mod io_device;
mod ram;
mod address_decoder;

use crate::clock::Clock;
use crate::ram::RAM;
use crate::address_decoder::AddressDecoder;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let mut decoder = AddressDecoder::new();
    decoder.add_device(
        0x2000..0x4000,
        Box::new(RAM::<0x2000>::new(None)),
    );
    
    let clock = Clock::new(2.0);
    let start = Instant::now();
    for tick in clock.wait {
        println!("tick: {:?}", tick.duration_since(start));
    }
}
