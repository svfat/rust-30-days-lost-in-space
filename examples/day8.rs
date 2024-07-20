#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Timer1Pwm, Timer2Pwm, PwmPinOps};
use arduino_hal::port::Pin;
use arduino_hal::port::mode::PwmOutput;
use panic_halt as _;

/// Delay between colour changes in milliseconds.
const COLOUR_DELAY: u16 = 500;

/// LED brightness levels.
const OFF: u8 = 0;
const DIM: u8 = 64;
const BRIGHTER: u8 = DIM + 64;
const BRIGHT: u8 = BRIGHTER + 64;
const BRIGHTEST: u8 = 255;

/// Type alias for a PWM output pin.
type PwmPin<TC, P> = Pin<PwmOutput<TC>, P>;

/// Struct representing an RGB LED with PWM control.
struct RgbLed<RedPin, GreenPin, BluePin, RedTc, GreenTc, BlueTc>
where
    RedPin: PwmPinOps<RedTc>,
    GreenPin: PwmPinOps<GreenTc>,
    BluePin: PwmPinOps<BlueTc>,
{
    red: PwmPin<RedTc, RedPin>,
    green: PwmPin<GreenTc, GreenPin>,
    blue: PwmPin<BlueTc, BluePin>,
}

impl<RedPin, GreenPin, BluePin, RedTc, GreenTc, BlueTc> RgbLed<RedPin, GreenPin, BluePin, RedTc, GreenTc, BlueTc>
where
    RedPin: PwmPinOps<RedTc>,
    GreenPin: PwmPinOps<GreenTc>,
    BluePin: PwmPinOps<BlueTc>,
{
    /// Creates a new `RgbLed` instance.
    fn new(
        red: PwmPin<RedTc, RedPin>,
        green: PwmPin<GreenTc, GreenPin>,
        blue: PwmPin<BlueTc, BluePin>,
    ) -> Self {
        Self { red, green, blue }
    }

    /// Enables the PWM outputs for the RGB LED.
    fn enable(&mut self) {
        self.red.enable();
        self.green.enable();
        self.blue.enable();
    }

    /// Sets the colour of the RGB LED.
    fn set_colour(&mut self, red_intensity: u8, green_intensity: u8, blue_intensity: u8) {
        self.red.set_duty(red_intensity);
        self.green.set_duty(green_intensity);
        self.blue.set_duty(blue_intensity);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut _serial = arduino_hal::default_serial!(dp, pins, 57600);

    let pwm_timer2 = Timer2Pwm::new(dp.TC2, arduino_hal::simple_pwm::Prescaler::Prescale64);
    let pwm_timer1 = Timer1Pwm::new(dp.TC1, arduino_hal::simple_pwm::Prescaler::Prescale64);

    let mut rgb_led = RgbLed::new(
        pins.d11.into_output().into_pwm(&pwm_timer2),
        pins.d10.into_output().into_pwm(&pwm_timer1),
        pins.d9.into_output().into_pwm(&pwm_timer1),
    );

    rgb_led.enable();

    let colour_sequence = [
        (OFF, OFF, OFF),           // Off
        (DIM, OFF, OFF),           // Dim Red
        (BRIGHTER, OFF, OFF),      // Brighter Red
        (BRIGHT, OFF, OFF),        // Bright Red
        (BRIGHTEST, OFF, OFF),     // Brightest Red
        (OFF, BRIGHTER, OFF),      // Green
        (OFF, OFF, BRIGHTER),      // Blue
        (BRIGHTER, BRIGHTER, OFF), // Yellow
        (OFF, BRIGHTER, BRIGHTER), // Cyan
        (BRIGHTER, OFF, BRIGHTER), // Magenta
        (BRIGHTER, BRIGHTER, BRIGHTER), // White
    ];

    loop {
        for &(red, green, blue) in &colour_sequence {
            rgb_led.set_colour(red, green, blue);
            arduino_hal::delay_ms(COLOUR_DELAY);
        }
    }
}

