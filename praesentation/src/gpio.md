# General Purpose Input/Output (GPIO)

Auf unserem Steckbrett wird `GPIO15` für die LED, `GPIO14` für den Taster verwendet.

Allgemeine Erklärung zu GPIOs: [elektronik-kompendium.de GPIO](https://www.elektronik-kompendium.de/sites/raspberry-pi/2611041.htm)

Information zur Verwendung von GPIOs mit Rust:
<https://docs.rs/rp2040-hal/latest/rp2040_hal/gpio/index.html>


Ansteuern der LED:

```rust
    let mut led_pin = pins.gpio15.into_push_pull_output();
```

# Beispiele

## Beispiel1: blinky mit clock delay

Bereits geklont. 
Betrachte die Implementierung der Verzögerung: `delay.delay_ms(500);`

### Beispiel2: blinky mit Rust Schleife

#### Beschreibung

Verwende zur Verzögerung einer Rust Schleife.

<!-- 
#### Beispielhafte Lösung

[beispiel_02_led_delay.rs](./rust-pico-linuxtag/src/bin/beispiel_02_led_delay.rs)

```rust, noplayground
    loop {
        for _ in 0..1_000_0000 {
            led_pin.set_high().unwrap();
        }
        for _ in 0..1_000_0000 {
            led_pin.set_low().unwrap();
        }
    }
```

-->


### Beispiel3: LED leuchtet nur wenn Taster gedrückt ist

Falls ihr schon früher die Lösung habt dürft ihr gerne kreativ werden und Euch selbst Aufgaben ausdenken. 

Eine Variante hierzu: Taster als Schalter: Durch Betätigen des Tasters wird die LED eingeschaltet, bei erneutem Drücken ausgeschaltet.


<!--

#### Lösung

[beispiel_03_button.rs](./rust-pico-linuxtag/src/bin/beispiel_03_button.rs)

```rust, noplayground
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

-->