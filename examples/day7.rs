#![no_std]
#![no_main]

use panic_halt as _;

const BATTERY_CAPACITY: u16 = 50000;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let photoresistor = pins.a0.into_analog_input(&mut adc);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut battery_level: u16 = 0;   
    
    loop {
        if battery_level < BATTERY_CAPACITY {
            // We will use integer arithmetic here
            let percentage: u16 = (battery_level as u32 * 100 / BATTERY_CAPACITY as u32) as u16;
            _ = ufmt::uwriteln!(&mut serial, "{}%\r\n", percentage); 
            battery_level += photoresistor.analog_read(&mut adc);
            if battery_level > BATTERY_CAPACITY {
                battery_level = BATTERY_CAPACITY;
                _ = ufmt::uwriteln!(&mut serial, "FULLY CHARGED");
            };
        };
        arduino_hal::delay_ms(100);
    }
}

