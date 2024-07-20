#![no_std]
#![no_main]

use panic_halt as _;

const MIN_DELAY: u16 = 50;
const MAX_DELAY: u16 = 500;

fn map(x: u16, in_min: u16, in_max: u16, out_min: u16, out_max: u16) -> u16 {
    if in_max == in_min {
        return out_min
    };
    let in_range = (in_max - in_min) as u64;
    let out_range = (out_max - out_min) as u64;
    let scaled_value = ((x - in_min) as u64 * out_range) / in_range;
    (scaled_value as u16) + out_min
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());


    let mut led = pins.d13.into_output();
    let photoresistor = pins.a0.into_analog_input(&mut adc);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut darkest_light: u16  = 1024;
    let mut brightest_light: u16 = 0;

    loop {
        let light_value: u16 = photoresistor.analog_read(&mut adc);
        _ = ufmt::uwriteln!(&mut serial, "Light value: {}\r\n", light_value);

        if light_value < darkest_light {
            darkest_light = light_value;
        };
        if light_value > brightest_light {
            brightest_light = light_value;
        };

        let delay_value = map(light_value, darkest_light, brightest_light, MIN_DELAY, MAX_DELAY);

        _ = ufmt::uwriteln!(&mut serial, "Delay value: {}\r\n", delay_value);

        led.toggle();
        arduino_hal::delay_ms(delay_value);
    }
}
