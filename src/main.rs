#![no_std]
#![no_main]

{% if device == "arduino-uno" %}
use arduino_uno as device;
{% endif %}{% if device == "arduino-leonardo" %}
use arduino_leonardo as device;
{% endif %}

use device::prelude::*;

// Pull in the panic handler from panic-halt
use panic_halt as _;

#[device::entry]
fn main() -> ! {
    let dp = device::Peripherals::take().unwrap();
{% if device == "arduino-uno" %}
    let mut pins = device::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
{% endif %}{% if device == "arduino-leonardo" %}
    let mut pins = device::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
{% endif %}

    loop {
        // TODO: Your code here
        // See examples at https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples
    }
}
