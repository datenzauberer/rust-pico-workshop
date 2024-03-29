use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{DynPinId, FunctionSioInput, FunctionSioOutput, Pin, PullDown, PullUp},
    pac,
    pwm::{Channel, FreeRunning, Pwm0, Slice, Slices, A},
    sio::Sio,
    timer::Timer,
    watchdog::Watchdog,
};
use core::result::{Result, Result::Ok};
use defmt_rtt as _;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;
use panic_probe as _;
use rp_pico as bsp; // board support package

pub struct MemoryBoard {
    leds: [Pin<DynPinId, FunctionSioOutput, PullDown>; 4],
    buttons: [Pin<DynPinId, FunctionSioInput, PullUp>; 4],
    buzzer: Channel<Slice<Pwm0, FreeRunning>, A>,
    seven_segment: [Pin<DynPinId, FunctionSioOutput, PullDown>; 8],
    delay: cortex_m::delay::Delay,
    timer: Timer,
}

impl MemoryBoard {
    pub fn new() -> Result<MemoryBoard, ()> {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

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

        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let led_array = [
            pins.gpio2.into_push_pull_output().into_dyn_pin(),
            pins.gpio3.into_push_pull_output().into_dyn_pin(),
            pins.gpio4.into_push_pull_output().into_dyn_pin(),
            pins.gpio5.into_push_pull_output().into_dyn_pin(),
        ];

        let button_array = [
            pins.gpio9.into_pull_up_input().into_dyn_pin(),
            pins.gpio8.into_pull_up_input().into_dyn_pin(),
            pins.gpio6.into_pull_up_input().into_dyn_pin(),
            pins.gpio7.into_pull_up_input().into_dyn_pin(),
        ];

        let seven_segment_array = [
            pins.gpio10.into_push_pull_output().into_dyn_pin(),
            pins.gpio11.into_push_pull_output().into_dyn_pin(),
            pins.gpio12.into_push_pull_output().into_dyn_pin(),
            pins.gpio13.into_push_pull_output().into_dyn_pin(),
            pins.gpio14.into_push_pull_output().into_dyn_pin(),
            pins.gpio15.into_push_pull_output().into_dyn_pin(),
            pins.gpio16.into_push_pull_output().into_dyn_pin(),
            pins.gpio17.into_push_pull_output().into_dyn_pin(),
        ];

        let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);

        let mut pwm = pwm_slices.pwm0;
        pwm.set_div_int(4);
        pwm.enable();

        let mut channel_a = pwm.channel_a;
        channel_a.output_to(pins.gpio0);
        channel_a.set_duty(0);

        Ok(MemoryBoard {
            leds: led_array,
            buttons: button_array,
            buzzer: channel_a,
            seven_segment: seven_segment_array,
            delay: delay,
            timer: timer,
        })
    }

    pub fn get_random_colour_number(&self) -> u8 {
        (self.timer.get_counter().duration_since_epoch().to_millis() % 4) as u8
    }

    pub fn wait_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms);
    }

    pub fn switch_led(&mut self, idx: u8, on: bool) {
        if (idx as usize) < self.leds.len() {
            if on {
                self.leds[idx as usize].set_high().unwrap();
            } else {
                self.leds[idx as usize].set_low().unwrap();
            }
        }
    }

    pub fn wait_for_button_press(&mut self) -> u8 {
        let mut active_button: Option<usize> = None;
        let mut previous_button_states = [1; 4]; // all buttons are high; TODO - replace "4"
        loop {
            for i in 0..self.buttons.len() {
                if self.buttons[i].is_low().unwrap() && previous_button_states[i] == 1 {
                    previous_button_states[i] = 0;
                    self.leds[i].set_high().unwrap();
                    active_button = Some(i);
                }
                if let Some(i) = active_button {
                    if self.buttons[i].is_high().unwrap() && previous_button_states[i] == 0 {
                        previous_button_states[i] = 1;
                        self.leds[i].set_low().unwrap();
                        return i as u8;
                    }
                }
            }
            self.wait_ms(10);
        }
    }

    pub fn sound(&mut self, ms: u32) {
        self.buzzer.set_duty(u16::MAX / 2);
        self.wait_ms(ms);
        self.buzzer.set_duty(0);
    }

    pub fn display(&mut self, number: u8) {
        let idx_list = match number {
            0 => [true, true, true, false, true, false, true, true],
            1 => [false, false, true, false, true, false, false, false],
            2 => [false, true, true, true, false, false, true, true],
            3 => [false, true, true, true, true, false, false, true],
            4 => [true, false, true, true, true, false, false, false],
            5 => [true, true, false, true, true, false, false, true],
            6 => [true, true, false, true, true, false, true, true],
            7 => [false, true, true, false, true, false, false, false],
            8 => [true, true, true, true, true, false, true, true],
            9 => [true, true, true, true, true, false, false, true],
            10 => [true, true, true, true, true, false, true, false],
            11 => [true, false, false, true, true, false, true, true],
            12 => [true, true, false, false, false, false, true, true],
            13 => [false, false, true, true, true, false, true, true],
            14 => [true, true, false, true, false, false, true, true],
            15 => [true, true, false, true, false, false, true, false],
            _ => [true, true, true, true, true, true, true, true],
        };

        for idx in 0..self.seven_segment.len() {
            if idx_list[idx] {
                self.seven_segment[idx].set_high().unwrap();
            } else {
                self.seven_segment[idx].set_low().unwrap();
            }
        }
    }
}
