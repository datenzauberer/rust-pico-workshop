# Workshop Embedded Setup

Es wird das Musterprojekt `rust-pico-linuxtag`, welches mit `probe-rs` gedebugged werden kann.

## Test Setup (und Debugging)

Projekt anlegen via shell:

```sh
#TODO: offizielles project-template verwendet embedded-hal v0.2 (statt 1.0)
# cargo generate https://github.com/rp-rs/rp2040-project-template
cargo generate https://github.com/datenzauberer/rp2040-project-template
```

Projektnamen: `rust-pico-linuxtag` eingeben und als `probe-rs` auswählen,
Der Konsolenoutput sollte in etwa wie folgt aussehen:

```
🔧   project-name: rust-pico-linuxtag ...
🔧   Generating template ...
? 🤷   Which flashing method do you intend to use? ›
❯ probe-rs
  elf2uf2-rs
  custom
  none

✔ 🤷   Which flashing method do you intend to use? · probe-rs
🔧   Moving generated files into: `.../rust-pico-linuxtag`...
🔧   Initializing a fresh Git repository
✨   Done! New project created ..../rust-pico-linuxtag
```

## Code Anpassung `src/main.rs`

Projekt in vscode öffnen, z.B. via shell:

```sh
code ./rust-pico-linuxtag
```

Datei `src/main.rs` editieren, led_pin ändern:

```rust
    let mut led_pin = pins.gpio15.into_push_pull_output();
```

```bash
cargo run
```

## Debugging

### launch.json anpassen (auf neues Binary)

Nun `launch.json` anpassen, ersetze `rp2040-project-template` mit `rust-pico-linuxtag`:

```
                    "programBinary": "target/thumbv6m-none-eabi/debug/rust-pico-linuxtag",
```

Der Binary Name kann auch mittels shell ermittelt werden: `ls -l target/thumbv6m-none-eabi/debug/`

Nun den Debugger starten in vscode mit `Ctrl-Shift-d` und dann Klicken auf den grünen Pfeil (links oben).

## Projektanalyse

Mit `cargo tree` werden die Projektabhängigkeiten angezeigt.

# Links

Wichtige Links zum Arbeiten mit dem Pico unter Rust:

 * Board Support Package: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico/examples
 * HAL: https://docs.rs/rp2040-hal/latest/rp2040_hal

Hilfreich ist das Klonen der Repos (diese können dann lokal durchsucht werden).
Wechselt in Euer OpenSource-Repo-Verzeichnis und:

```sh
git clone https://github.com/rp-rs/rp-hal-boards
git clone https://github.com/rp-rs/rp-hal

# Beispiele sind unter examples zu finden:
ls ./rp-hal-boards/boards/rp-pico/examples
ls ./rp-hal/rp2040-hal/examples
```

# Workshop Beispiele

## General Purpose Input and Output (GPIO)

Auf unserem Steckbrett wird GP15 für die LED, GP14 für den Taster verwendet.

Information zur Verwendung von GPIOs mit Rust:
https://docs.rs/rp2040-hal/latest/rp2040_hal/gpio/index.html


Ansteuern der LED:

```rust
    let mut led_pin = pins.gpio15.into_push_pull_output();
```

### Beispiel1: blinky mit clock delay

Bereits geklonet. `delay.delay_ms(500);` verzögert.

### Beispiel2: blinky mit Rust Schleife

#### Beschreibung

Verwendet zur Verzögerung einer Rust Schleife.

#### Beispielhafte Lösung

```rust
    loop {
        for _ in 0..1_000_0000 {
            led_pin.set_high().unwrap();
        }
        for _ in 0..1_000_0000 {
            led_pin.set_low().unwrap();
        }
    }
```

### Beispiel3: LED leuchtet nur wenn Taster gedrückt ist

Falls ihr schon früher die Lösung habt dürft ihr gerne kreativ werden und Euch selbst Aufgaben ausdenken. Eine Variante hierzu z.B. 
Taster als Schalter: Durch Betätigen des Tasters wird die LED eingeschaltet, bei erneutem Drücken ausgeschaltet.

#### Lösung

```rust
    loop {
        if button_pin.is_high().unwrap() {
            // Button is pressed (active low)
            info!("button on!");
            led_pin.set_high().unwrap();
        } else {
            info!("button off!");
            led_pin.set_low().unwrap();
        }
    }
```

## PWM

### Beispiel4: LED mit PWM ansteuern

Wichtige Information: LED ist auf GPIO15. 
Hierzu muss aus dem Dateblatt die Slice und die Channel ermittelt werden. Ansonsten kann das rp2040-hal SMTODO Beispiel verwendet werden.


#### Lösung

```rust
        // A shorter alias for the Hardware Abstraction Layer, which provides
        // higher-level drivers.
        use rp_pico::hal;

        // GPIO traits
        use embedded_hal::pwm::SetDutyCycle;

        // The minimum PWM value (i.e. LED brightness) we want
        const LOW: u16 = 0;

        // The maximum PWM value (i.e. LED brightness) we want
        const HIGH: u16 = 25000;

        // Init PWMs
        let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        // Configure PWM7
        let pwm = &mut pwm_slices.pwm7;
        pwm.set_ph_correct();
        pwm.enable();

        // Output channel B on PWM4 to the LED pin
        let channel = &mut pwm.channel_b;
        channel.output_to(led_pin);
        // Infinite loop, fading LED up and down
        loop {
            // Ramp brightness up
            for i in (LOW..=HIGH).skip(100) {
                delay.delay_us(8);
                let _ = channel.set_duty_cycle(i);
            }

            // Ramp brightness down
            for i in (LOW..=HIGH).rev().skip(100) {
                delay.delay_us(8);
                let _ = channel.set_duty_cycle(i);
            }

            delay.delay_ms(500);
        }
```

## ADC 

### Beispiel5: Temperature sensor in free-running mode

siehe: https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo

### Beispiel6: AnalogWert von Potentiometer lesen

wie https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo
jedoch wird der ADC0 gelesen:

```rust
        // Configure GPIO26 as an ADC input
        let mut adc_pin_0 = rp2040_hal::adc::AdcPin::new(pins.gpio26).unwrap();
```

#### Lösung:

```rust
        // EXAMPLE06: ADC sensor in free-running mode
        // https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo
        use rp2040_hal::{adc::Adc, gpio::Pins, pac, Sio};
        // Enable adc
        let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
        // Configure GPIO26 as an ADC input
        let mut adc_pin_0 = rp2040_hal::adc::AdcPin::new(pins.gpio26).unwrap();

        // Configure & start capturing to the fifo:
        let mut fifo = adc
            .build_fifo()
            .clock_divider(0, 0) // sample as fast as possible (500ksps. This is the default)
            .set_channel(&mut adc_pin_0)
            .start();

        loop {
            if fifo.len() > 0 {
                // Read one captured ADC sample from the FIFO:
                let adc_counts: u16 = fifo.read();
                info!("adc0 value: {}", adc_counts);
            }
        }
```

## IRQ 

### Beispiel7: blinky mit Interrupt gesteuerten Timer

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

// SMINFO: use crate::pac::interrupt; = use bsp::hal::pac::interrupt;
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