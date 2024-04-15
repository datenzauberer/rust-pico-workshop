# Workshop Ablauf: hal-blinky

## zum Test Setup (und Debugging) <https://github.com/rp-rs/rp-hal> RP2040

```.bash
cargo generate https://github.com/rp-rs/rp2040-project-template
```

src/main.rs: adjust LED pin

```.rust
//    let mut led_pin = pins.led.into_push_pull_output();
    let mut led_pin = pins.gpio16.into_push_pull_output();
```

``` {.bash org-language="sh"}
cargo run
```

### launch.json anpassen (auf neues Binary)

\"programBinary\":
\"target/thumbv6m-none-eabi/debug/rp2040-project-template\", -\>
\"programBinary\": \"target/thumbv6m-none-eabi/debug/myhal\",

Binary Name kann in der bash ermittelt werden: (im projectroot):

``` {.bash org-language="sh"}
ls -l target/thumbv6m-none-eabi/debug/
```