#![no_std]
#![no_main]

use arduino_hal::hal::wdt;
use arduino_hal::prelude::*;
use embedded_hal::serial::Read;
// use panic_halt as _;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts
    // avr_device::interrupt::disable();

    let dp = unsafe {
        arduino_hal::Peripherals::steal()
    };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(&mut serial, " At {}:{}:{}\r", loc.file(), loc.line(), loc.column()).void_unwrap();
    }

    let mut led = pins.d11.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut red_led_driver = pins.d11.into_output();
    let mut green_led_driver = pins.d10.into_output();
    let mut blue_led_driver = pins.d9.into_output();
    let mut count: i8 = 0;

    for i in 0..20 {
        red_led_driver.toggle();
        arduino_hal::delay_ms(100);
    }

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms2000);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello, world!\r").void_unwrap();
    // panic!();

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

        watchdog.stop();
        let mut b = nb::block!(serial.read()).void_unwrap();

        let terminator: u8 = 10;
        let at: u8 = 64;
        while b != terminator {
            b = nb::block!(serial.read()).void_unwrap();

            if b == at {
                blue_led_driver.toggle();
                arduino_hal::delay_ms(1000);
                blue_led_driver.toggle();
                arduino_hal::delay_ms(1000);
                blue_led_driver.toggle();
                arduino_hal::delay_ms(1000);
            }
        }

        // let b = nb::await!(serial.read()).void_unwrap();
        watchdog.start(wdt::Timeout::Ms2000);
        ufmt::uwriteln!(&mut serial, "> {}\r", b);
        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();

        // feeding watchdog timer, if left unfed,
        // watchdog will reset the device.
        // This is used to ensure the firmware does not lock itself up.
        watchdog.feed();
    }
}
