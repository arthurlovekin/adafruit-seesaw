#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use adafruit_seesaw::{devices::JoyFeatherWing, prelude::*, SeesawDriver};
use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use {esp_backtrace as _, esp_println as _};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // STEMMA QT pins on the Adafruit Feather ESP32-C6: SDA=GPIO19, SCL=GPIO18.
    // Adjust if you are wiring the FeatherWing on a different board.
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .expect("failed to create I2C")
    .with_sda(peripherals.GPIO19)
    .with_scl(peripherals.GPIO18);

    let driver = SeesawDriver::new(Delay::new(), i2c);

    let mut wing = JoyFeatherWing::new_with_default_addr(driver)
        .init()
        .expect("Failed to initialize Joy FeatherWing");

    info!("Joy FeatherWing initialized");

    loop {
        let (x, y) = wing.joystick().unwrap_or((0, 0));
        let b = wing.buttons().unwrap_or(adafruit_seesaw::devices::JoyButtons {
            up: false,
            down: false,
            left: false,
            right: false,
            select: false,
        });
        info!(
            "x={=u16} y={=u16}  U={=bool} D={=bool} L={=bool} R={=bool} SEL={=bool}",
            x, y, b.up, b.down, b.left, b.right, b.select
        );

        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(100) {}
    }
}
