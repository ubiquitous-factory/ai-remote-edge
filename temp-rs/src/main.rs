#![no_std]
#![no_main]

use arduino_hal::prelude::*;

use micromath::F32;
use panic_halt as _;
use ufmt_float::uFmt_f32;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut led = pins.d13.into_output();
    let a0 = pins.a0.into_analog_input(&mut adc);

    loop {
        led.toggle(); // Blinking to show activity

        // Create an array of sensors to this is exstensbile
        let values = [a0.analog_read(&mut adc)];
        let r1: f32 = 10000.; // The resistor value

        // c1, c2, & c3 = Coefficients used for the equation (numbers vary depending on the model and style of the NTC thermistor. Range of temperature is also an important factor).
        // https://www.sensorsci.com/understanding-the-steinhart-hart-equation-and-how-to-use-it
        // https://en.wikipedia.org/wiki/Steinhart%E2%80%93Hart_equation
        let a: f32 = 0.001129148;
        let b: f32 = 0.000234125;
        let c: f32 = 0.0000000876741;

        for (i, v) in values.iter().enumerate() {
            let r2 = r1 * (1024.0 / *v as f32 - 1.);
            let log_r2 = F32(r2).ln();
            let temp_k = 1.0 / (a + b * log_r2 + c * log_r2 * log_r2 * log_r2); //  Kelvin
            let temp_c = temp_k - 273.15; // Celcius
            let temp_f = (temp_c * 9.0) / 5.0 + 32.0; // Fahrenheit
            let temp_k_write = uFmt_f32::Three(f32::from(temp_k));
            let temp_c_write = uFmt_f32::Three(f32::from(temp_c));
            let temp_f_write = uFmt_f32::Three(f32::from(temp_f));

            ufmt::uwrite!(
                &mut serial,
                "{} A{}; {} K; {} C; {} F",
                v,
                i,
                temp_k_write,
                temp_c_write,
                temp_f_write
            )
            .void_unwrap();
        }

        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
