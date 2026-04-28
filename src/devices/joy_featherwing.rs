use super::SeesawDeviceInit;
use crate::{
    modules::{
        adc::AdcModule,
        gpio::{GpioModule, PinMode},
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  /// Adafruit Joy FeatherWing
  ///
  /// 2-axis analog joystick + 5 momentary buttons (Up/Down/Left/Right/Select).
  /// Communicates over I2C at default address `0x49`.
  name: JoyFeatherWing,
  hardware_id: HardwareId::SAMD09,
  product_id: 3632,
  default_addr: 0x49
}

/// Joystick X is read from ADC pin 2 on the SAMD09.
const JOYSTICK_X_PIN: u8 = 2;
/// Joystick Y is read from ADC pin 3 on the SAMD09.
const JOYSTICK_Y_PIN: u8 = 3;

pub const BUTTON_RIGHT: u8 = 6;
pub const BUTTON_DOWN: u8 = 7;
pub const BUTTON_LEFT: u8 = 9;
pub const BUTTON_UP: u8 = 10;
pub const BUTTON_SEL: u8 = 14;

const BUTTON_MASK: u32 = (1 << BUTTON_RIGHT)
    | (1 << BUTTON_DOWN)
    | (1 << BUTTON_LEFT)
    | (1 << BUTTON_UP)
    | (1 << BUTTON_SEL);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct JoyButtons {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub select: bool,
}

impl JoyButtons {
    fn from_bulk(pins: u32) -> Self {
        // Buttons are active-low (pulled up to 3.3V; pressed pulls to ground).
        let pressed = |pin: u8| (pins >> pin) & 0x1 == 0;
        Self {
            up: pressed(BUTTON_UP),
            down: pressed(BUTTON_DOWN),
            left: pressed(BUTTON_LEFT),
            right: pressed(BUTTON_RIGHT),
            select: pressed(BUTTON_SEL),
        }
    }
}

impl<D: Driver> AdcModule<D> for JoyFeatherWing<D> {}
impl<D: Driver> GpioModule<D> for JoyFeatherWing<D> {}

impl<D: Driver> SeesawDeviceInit<D> for JoyFeatherWing<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        self.enable_button_pins()?;
        Ok(self)
    }
}

impl<D: Driver> JoyFeatherWing<D> {
    pub fn enable_button_pins(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode_bulk(BUTTON_MASK, PinMode::InputPullup)
    }

    pub fn enable_button_interrupts(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.interrupt_enable_bulk(BUTTON_MASK)
    }

    pub fn disable_button_interrupts(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.interrupt_disable_bulk(BUTTON_MASK)
    }

    /// Read the joystick X axis. Returns a 10-bit value (0..=1023).
    ///
    /// The joystick is wired such that the raw value increases as the stick
    /// moves left; see [`Self::joystick`] for a normalized reading.
    pub fn joystick_x_raw(&mut self) -> Result<u16, SeesawError<D::Error>> {
        self.analog_read(JOYSTICK_X_PIN)
    }

    /// Read the joystick Y axis. Returns a 10-bit value (0..=1023).
    pub fn joystick_y_raw(&mut self) -> Result<u16, SeesawError<D::Error>> {
        self.analog_read(JOYSTICK_Y_PIN)
    }

    /// Read both joystick axes, with the convention used by the Adafruit
    /// example: `(1023 - raw)` so that increasing X is right and increasing Y
    /// is up.
    pub fn joystick(&mut self) -> Result<(u16, u16), SeesawError<D::Error>> {
        let x = self.joystick_x_raw()?;
        let y = self.joystick_y_raw()?;
        Ok((1023u16.saturating_sub(x), 1023u16.saturating_sub(y)))
    }

    /// Read all five buttons in a single GPIO bulk read.
    pub fn buttons(&mut self) -> Result<JoyButtons, SeesawError<D::Error>> {
        self.digital_read_bulk().map(JoyButtons::from_bulk)
    }
}
