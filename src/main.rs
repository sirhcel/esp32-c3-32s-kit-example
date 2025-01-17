use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;

use anyhow::*;
use log::*;

use embedded_hal::digital::blocking::{InputPin, OutputPin};
use embedded_hal::pwm::blocking::PwmPin;
use esp_idf_hal::peripherals::Peripherals;

use esp_idf_hal::ledc::{
    config::TimerConfig,
    Channel,
    Timer,
};
use esp_idf_hal::prelude::*;

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Let's start the board example!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // let mut red_led = pins.gpio3.into_output()?;
    let mut green_led = pins.gpio4.into_output()?;
    let mut blue_led = pins.gpio5.into_output()?;
    let mut amber_led = pins.gpio18.into_output()?;
    let mut white_led = pins.gpio19.into_output()?;
    let button = pins.gpio9.into_input().expect("Built-in button failed");

    // experimental esp_idf_hal PWM: Create a 25 kHz PWM signal with 25 % duty cycle on red led
    let config = TimerConfig::default().frequency(25.kHz().into());
    let timer = Timer::new(peripherals.ledc.timer0, &config)?;
    let mut channel = Channel::new(peripherals.ledc.channel0, &timer, pins.gpio3)?;
    channel.set_duty(64)?;

    loop {
        amber_led.set_high()?;
        thread::sleep(Duration::from_millis(1000));
        amber_led.set_low()?;
        white_led.set_high()?;
        thread::sleep(Duration::from_millis(1000));
        white_led.set_low()?;
        // red_led.set_high()?;
        // thread::sleep(Duration::from_millis(1000));
        // red_led.set_low()?;
        green_led.set_high()?;
        thread::sleep(Duration::from_millis(1000));
        green_led.set_low()?;
        blue_led.set_high()?;
        thread::sleep(Duration::from_millis(1000));
        blue_led.set_low()?;
        if button.is_low().unwrap() {
            info!("Button pressed!");
        }
    }
}
