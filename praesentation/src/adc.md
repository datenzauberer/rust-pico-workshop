# Analog-to-digital converter (ADC)

Nützliche Links:

[elektronik-kompendium.de: ADC](https://www.elektronik-kompendium.de/sites/raspberry-pi/2701241.htm)

[rohm.de: ad-da-converters](https://www.rohm.de/electronics-basics/ad-da-converters/what-are-ad-da-converters)

# Beispiele

## Beispiel5: Temperature sensor in free-running mode

siehe: <https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo>

## Beispiel6: AnalogWert von Potentiometer lesen

Wie oben nur wird der `ADC0` gelesen:

```rust
        // Configure GPIO26 as an ADC input
        let mut adc_pin_0 = rp2040_hal::adc::AdcPin::new(pins.gpio26).unwrap();
```

