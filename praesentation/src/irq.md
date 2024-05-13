# Interrupt Requests (IRQ)

[elektronik-kompendium.de Pico IRQs](https://www.elektronik-kompendium.de/sites/raspberry-pi/2710121.htm)

# Beispiel7: blinky mit Interrupt gesteuerten Timer

<!--
```rust
    #[cfg(feature = "feature_example_07")]
    {
        // EXAMPLE07: Alarm with periodic interrupt
        use bsp::hal::fugit::MicrosDurationU32;
        use rp2040_hal as hal;
        use rp2040_hal::timer::Alarm;

        let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
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
```
-->