#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d12.into_output();

    loop {
        led.set_high();
        arduino_hal::delay_ms(1000);
        led.set_low();
        arduino_hal::delay_ms(1000);
    }

    // The another variant is .toggle();
    // loop {
    //    led.toggle();
    //    arduino_hal::delay_ms(1000);
    // }
}
