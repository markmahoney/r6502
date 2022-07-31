pub mod address_decoder;
pub mod cpu;
pub mod clock;
pub mod io_device;
pub mod memory;

use crate::address_decoder::AddressDecoder;
use crate::clock::Clock;
use crate::cpu::cpu_6502::CPU6502;
use crate::io_device::IODevice;
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;

fn print_state(cpu: &CPU6502, address_line: &AddressDecoder) {
    println!("Status register: {:#010b}", cpu.registers.status.flags);
    println!("Accumulator: {:#04x}", cpu.registers.a);
    println!("Data bus: {:#04x}", cpu.registers.data);
    println!("ADH/ADL: {:#04x} {:#04x}", cpu.registers.adh, cpu.registers.adl);
    println!("Contents of 0x6100 to 0x6102: {:#04x} {:#04x} {:#04x}",
             address_line.get(0x6100),
             address_line.get(0x6101),
             address_line.get(0x6102)
    );
}

fn main() {
    let clock = Clock::new(1000000.0);
    let mut address_line = AddressDecoder::new();
    let mut cpu = CPU6502::new();

    address_line.add_device(
        0x0000..=0x7FFF,
        Box::new(
            RAM::<0x8000>::new_from_file("test_bin/add.bin").expect("error setting ram from file")
        ),
    );
    
    address_line.add_device(
        0x8000..=0xFFFF,
        Box::new(ROM::<0x8000>::new(Some([255; 0x8000]))),
    );

    // Go until we reach an opcode we don't know how to run
    clock.start(|| {
        println!("\n=== Clock Cycle ===");
        let cont = cpu.tick(&mut address_line);
        print_state(&cpu, &address_line);
        cont
    });
}
