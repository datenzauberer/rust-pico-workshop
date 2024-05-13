//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    watchdog::Watchdog,
};

// EXAMPLE07: Alarm with periodic interrupt
use bsp::hal::fugit::MicrosDurationU32;
use rp2040_hal::timer::Alarm;
const BLINK_INTERVAL_US: MicrosDurationU32 = MicrosDurationU32::secs(1);

// Give our LED and Alarm a type alias to make it easier to refer to them
// TODO: When changing Pin Gpio28 to Gpio16 it has to be done on two places. How to avoid this ?
type LedAndAlarm = (
    //    hal::gpio::Pin<hal::gpio::bank0::Gpio28, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
    rp2040_hal::gpio::Pin<
        rp2040_hal::gpio::bank0::Gpio15,
        rp2040_hal::gpio::FunctionSioOutput,
        rp2040_hal::gpio::PullDown,
    >,
    rp2040_hal::timer::Alarm0,
);

use core::cell::RefCell;
use critical_section::Mutex;

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;
// Place our LED and Alarm type in a static variable, so we can access it from interrupts
static mut LED_AND_ALARM: Mutex<RefCell<Option<LedAndAlarm>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    info!("Program start:");
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    use bsp::hal::fugit::MicrosDurationU32;
    use rp2040_hal as hal;
    use rp2040_hal::timer::Alarm;

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Period that each of the alarms will be set for - 1 second and 300ms respectively
    const BLINK_INTERVAL_US: MicrosDurationU32 = MicrosDurationU32::secs(1);
    critical_section::with(|cs| {
        let mut alarm = timer.alarm_0().unwrap();
        // Schedule an alarm in 1 second
        let _ = alarm.schedule(BLINK_INTERVAL_US);
        // Enable generating an interrupt on alarm
        alarm.enable_interrupt();
        // Move alarm into ALARM, so that it can be accessed from interrupts
        unsafe {
            LED_AND_ALARM.borrow(cs).replace(Some((led_pin, alarm)));
        }
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
    }

    loop {
        // Wait for an interrupt to fire before doing any more work
        cortex_m::asm::wfi();
    }
}

use bsp::hal::pac::interrupt;
#[allow(non_snake_case)]
#[interrupt]
fn TIMER_IRQ_0() {
    critical_section::with(|cs| {
        // Temporarily take our LED_AND_ALARM
        let ledalarm = unsafe { LED_AND_ALARM.borrow(cs).take() };
        if let Some((mut led, mut alarm)) = ledalarm {
            // Clear the alarm interrupt or this interrupt service routine will keep firing
            alarm.clear_interrupt();
            // Schedule a new alarm after BLINK_INTERVAL_US have passed (1 second)
            let _ = alarm.schedule(BLINK_INTERVAL_US);
            // Blink the LED so we know we hit this interrupt

            // SMINFO: embedded_hal::digital::ToggleableOutputPin is in
            // embedded-hal = 0.2 with feature: unproven therefore we don't use it
            // https://docs.rs/embedded-hal/0.2.2/embedded_hal/digital/trait.ToggleableOutputPin.html
            // embedded_hal::digital::ToggleableOutputPin::toggle(&mut led).unwrap();
            led.toggle();

            // Return LED_AND_ALARM into our static variable
            unsafe {
                LED_AND_ALARM
                    .borrow(cs)
                    .replace_with(|_| Some((led, alarm)));
            }
        }
    });
}
