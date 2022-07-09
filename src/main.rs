pub mod address_decoder;
pub mod cpu;
pub mod clock;
pub mod io_device;
pub mod memory;

use crate::address_decoder::AddressDecoder;
use crate::clock::Clock;
use crate::cpu::cpu_6502::CPU6502;
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;

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

    let mut cpu = CPU6502::new();
    let clock = Clock::new(1.0);

    clock.start(|| {
        cpu.tick(&mut decoder);
        true
    })
}
