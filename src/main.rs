// mod cpu;
mod ppu;
mod apu;
mod memory;

fn main() {
    println!("Welcome to the NES Emulator!");

    // Initialize components
    //let mut cpu = cpu::CPU::new();
    let mut ppu = ppu::PPU::new();
    let mut apu = apu::APU::new();
    let mut memory = memory::Memory::new();

    // Main emulation loop
    loop {
        //cpu.step(&mut memory);
        ppu.step();
        apu.step();

        // Add logic for synchronization, input handling, and rendering
    }
}