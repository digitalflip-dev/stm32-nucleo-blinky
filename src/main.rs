#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::delay::Delay;

use hal::{
    self,
    clocks::{Clocks},
    gpio::{
        Pin,
        PinMode,
        Port
    },
    pac
};

use defmt_rtt as _;
//use panic_halt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {

    // Set up CPU peripherals
    let cp = cortex_m::Peripherals::take().unwrap();

    // Set up microcontroller peripherals
    let _dp = pac::Peripherals::take().unwrap();

    defmt::println!("Hello World");

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();
    
    // Setup a delay based on Cortex-M systick
    let mut delay = Delay::new(cp.SYST,  clock_cfg.systick());

    
    let mut led = Pin::new(Port::A, 5, PinMode::Output);

    led.set_low();
    
    loop {
        led.set_high();
        delay.delay_ms(200);
        led.set_low();
        delay.delay_ms(200);
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
     cortex_m::asm::udf()
}