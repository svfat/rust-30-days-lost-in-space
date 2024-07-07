#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d12.into_output();
    let switch = pins.d2.into_floating_input();

    loop {
        if switch.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
