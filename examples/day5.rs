#![no_std]
#![no_main]

use panic_halt as _;

const MIN_DELAY: u16 = 50;
const MAX_DELAY: u16 = 500;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());


    let mut led = pins.d13.into_output();
    let photoresistor = pins.a0.into_analog_input(&mut adc);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    loop {
        let light_value = photoresistor.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Light value: {}", light_value);
    }
}
