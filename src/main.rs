pub mod address_decoder;
pub mod clock;
pub mod io_device;
pub mod memory;

use crate::address_decoder::AddressDecoder;
use crate::clock::Clock;
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;
use std::thread;

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

    // Clock updates every second, which means each line level changes state every half second.
    let mut clock = Clock::new(1.0);
    let device1 = clock.connect_phase1();
    let device2 = clock.connect_phase2();

    thread::spawn(move || {
        for line_level in device1 {
            println!("device1 clock line level: {:?}", line_level);
        }
    });

    thread::spawn(move || {
        for line_level in device2 {
            println!("device2 clock line level: {:?}", line_level);
        }
    });

    clock.start().join().expect("???");
}
