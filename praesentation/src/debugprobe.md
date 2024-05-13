# Picoprobe setup

Two Raspberry Pi Picos are used. PicoA operates as a debug probe, while PicoB serves as the production probe (the target hosting your code). Details in [Getting Started with Pico - Appendix A: Using Picoprobe](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf#page=62).

## Flash debug probe (PicoA)

Download the firmware **`debugprobe_on_pico.uf2`** from:

<https://github.com/raspberrypi/debugprobe/releases>

ATTENTION: **debugprobe_on_pico.uf2** is needed !! 

(Alternatively build the debugprobe from source code as described "Getting started with Pico - Build and flash picoprobe")

Boot the Raspberry Pi PicoAwith the BOOTSEL button pressed and copy the firmware, e.g.

```sh
sudo cp ~/Downloads/debugprobe_on_pico.uf2 /media/michael/RPI-RP2
```

## Picoprobe Wiring

### Debug Wiring (SWD and UART bridge)

| PicoA |              | PicoB  |              |
|------:|--------------|-------:|--------------|
| Pin   | Description  | Pin    | Description  |
| 38    | GND          | Debug2 | GND          |
| 4     | GP2          | Debug1 | SWCLK        |
| 5     | GP3          | Debug3 | SWDIO        |
| 6     | GP4/UART1 TX |      2 | GP1/UART0 RX |
| 7     | GP5/UART1 RX |      1 | GP1/UART0 RX |

**Note**: DebugPins are numbered from left to right when the USB connector is facing up.

### Power Supply (Optional)

One advantage of using a Pico as a debug probe is that no separate power supply for the PicoB is needed:

| PicoA |             | PicoB |             |
|------:|-------------|------:|-------------|
|   Pin | Description |   Pin | Description |
|    39 | VSYS        |    39 | VSYS        |

Of course It's also possible to provide the power the PicoB via USB. 

## Testing Picoprobe

Test the connection to the probe

```sh
probe-rs list
```

This should return a output like:

```
The following debug probes were found:
[0]: Debugprobe on Pico (CMSIS-DAP) (VID: 2e8a, PID: 000c, Serial: E661A4D417595929, CmsisDap)
```

Now test with flashing a simple application.
