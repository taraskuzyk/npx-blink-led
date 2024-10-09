#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]

fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    //peripherals.porte.pcr29().write(|w| w.pe().b1());
    loop {
        //for _ in 0..4_000_000 {
        //    nop();
        //}
    }
}
