#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut red_led_driver = pins.d11.into_output();
    let mut green_led_driver = pins.d10.into_output();
    let mut blue_led_driver = pins.d9.into_output();
    let mut count: i8 = 0;

    loop {
        led.toggle();

        if count > 2 {
            count = 0;
        }

        if count == 0 {
            red_led_driver.toggle();
        } else if count == 1 {
            green_led_driver.toggle();
        } else if count == 2 {
            blue_led_driver.toggle();
        }

        count += 1;

        arduino_hal::delay_ms(1000);
    }
}
