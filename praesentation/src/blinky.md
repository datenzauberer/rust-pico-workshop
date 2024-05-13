# Projekterstellung

Wir legen ein Musterprojekt namens `rust-pico-linuxtag` an, starten es und debuggen es ein bisschen.

## Testaufbau und Debugging

### Projekt anlegen via shell:

Nutze den folgenden Befehl in der Shell, um das Projekt zu erstellen:

```sh
cargo generate https://github.com/datenzauberer/rp2040-project-template
# offizielles project-template verwendet embedded-hal v0.2 (statt 1.0)
# cargo generate https://github.com/rp-rs/rp2040-project-template
```

Wähle `rust-pico-linuxtag` als Projektnamen und `probe-rs` für das Flashen. Der Output sollte etwa so aussehen:

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

## Code anpassung in `src/main.rs`

Öffne das Projekt in VSCode:
, z.B. via Shell:

```sh
code ./rust-pico-linuxtag
```

Ändere in `src/main.rs` den LED-Pin:

```rust
    let mut led_pin = pins.gpio15.into_push_pull_output();
```

Und dann starten wir:

```bash
cargo run
```

## Debugging

### Anpassung der `launch.json` für das neue Binary

Nun `launch.json` anpassen, ersetze `rp2040-project-template` mit `rust-pico-linuxtag`:

```
                    "programBinary": "target/thumbv6m-none-eabi/debug/rust-pico-linuxtag",
```

Den Namen des Binaries kannst Du  auch mittels Shell ermittelt: `ls -l target/thumbv6m-none-eabi/debug/`

Debugger in VSCode starten mit `Ctrl-Shift-D` und zum Starten auf den grünen Pfeil klicken. 

## Projektanalyse

Mit `cargo tree` werden die Projektabhängigkeiten angezeigt.

# Nützliche Links

Hier ein paar nützliche Links für die Arbeit mit dem Pico unter Rust:

- **Board Support Package**: [RP2040 HAL Boards GitHub](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico/examples)
- **HAL**: [rp2040-hal auf docs.rs](https://docs.rs/rp2040-hal/latest/rp2040_hal/)

 * Board Support Package: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico/examples
 * HAL: https://docs.rs/rp2040-hal/latest/rp2040_hal

Um die Repos zu klonen, geh in Repo-Verzeichnis:

```sh
git clone https://github.com/rp-rs/rp-hal-boards
git clone https://github.com/rp-rs/rp-hal

# Beispiele sind unter examples zu finden:
ls ./rp-hal-boards/boards/rp-pico/examples
ls ./rp-hal/rp2040-hal/examples
```
