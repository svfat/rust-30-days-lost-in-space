#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::port::mode::{Input, Output, Floating};

fn on_off<S: PinOps, L: PinOps>(switch: &Pin<Input<Floating>, S>, led: &mut Pin<Output, L>) {
    if switch.is_high() {
        led.set_high();
    } else {
        led.set_low();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut green_led = pins.d12.into_output();
    let mut blue_led = pins.d11.into_output();
    let mut red_led = pins.d10.into_output();

    let green_switch = pins.d4.into_floating_input();
    let blue_switch = pins.d3.into_floating_input();
    let red_switch = pins.d2.into_floating_input();

    loop {
        on_off(&red_switch, &mut red_led);
        on_off(&blue_switch, &mut blue_led);
        on_off(&green_switch, &mut green_led);
    }
}
