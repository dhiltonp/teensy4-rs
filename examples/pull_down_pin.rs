//! Configure pin 6 with a pull-down resistor, and sample it to control
//! whether the LED is flashing or not.

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt::entry;
use imxrt_hal::{
    gpio::GPIO,
    iomuxc::{self, Config, Hysteresis, PullKeeper},
};
use teensy4_bsp::{configure_led, t40, Peripherals, SysTick};

// The pin configuration can be defined at compile time,
// and at run time. This example uses a constant, so the
// configuration is defined at compile time.
const SWITCH_CONFIG: Config = Config::zero()
    .set_hysteresis(Hysteresis::Enabled)
    .set_pull_keeper(Some(PullKeeper::Pulldown100k));

const LED_PERIOD_MS: u32 = 500;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut systick = SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = t40::into_pins(p.iomuxc);
    let mut led = configure_led(pins.p13);

    let mut switch_pin = pins.p6;

    iomuxc::configure(&mut switch_pin, SWITCH_CONFIG);

    let switch_gpio = GPIO::new(switch_pin);

    loop {
        if switch_gpio.is_set() {
            led.toggle()
        }
        systick.delay(LED_PERIOD_MS);
    }
}
