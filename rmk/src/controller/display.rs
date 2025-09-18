use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::{PrimitiveStyle, Rectangle, StyledDrawable}};
use embedded_hal::i2c::I2c;
use ssd1306::prelude::*;

use crate::{channel::{ControllerSub, CONTROLLER_CHANNEL}, event::ControllerEvent};

use super::{Controller, PollingController};

// let twim = embassy_nrf::twim::Twim::new(p.TWISPI0, Irqs, p.P0_17, p.P0_20, Default::default(), &mut []);
// let display = rmk::controller::display::Ssd1306Backend::new(twim);
// let mut controller = rmk::controller::display::DisplayController::new(display);

pub struct Ssd1306Backend<DI: WriteOnlyDataCommand, SIZE: DisplaySize> {
    display: ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>,
}

impl<I: I2c> Ssd1306Backend<I2CInterface<I>, DisplaySize128x32> {
    pub fn new(i2c: I) -> Self {
        let mut display = ssd1306::Ssd1306::new(ssd1306::I2CDisplayInterface::new(i2c), ssd1306::size::DisplaySize128x32, ssd1306::rotation::DisplayRotation::Rotate90)
            .into_buffered_graphics_mode();
        display.init();
        Self {
            display,
        }
    }
}

impl<DI: WriteOnlyDataCommand, SIZE: DisplaySize> DisplayBackend for Ssd1306Backend<DI, SIZE> {
    type Target = ssd1306::Ssd1306<DI, SIZE, ssd1306::mode::BufferedGraphicsMode<SIZE>>;

    fn clear(&mut self) {
        self.display.clear_buffer();
    }

    fn flush(&mut self) {
        self.display.flush();
    }

    fn target(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}

pub trait DisplayBackend {
    type Target: DrawTarget<Color = BinaryColor, Error = display_interface::DisplayError>;

    fn clear(&mut self);
    fn flush(&mut self);

    fn target(&mut self) -> &mut Self::Target;
}

pub struct DisplayController<D: DisplayBackend> {
    sub: ControllerSub,
    display: D,
    on: bool,
}

impl<D: DisplayBackend> DisplayController<D> {
    pub fn new(display: D) -> Self {
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            display,
            on: true,
        }
    }
}

impl<D: DisplayBackend> Controller for DisplayController<D> {
    type Event = ControllerEvent;

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }

    async fn process_event(&mut self, event: Self::Event) {
        self.on = !self.on;
    }
}

impl<D: DisplayBackend> PollingController for DisplayController<D> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_hz(60);

    async fn update(&mut self) {
        self.display.clear();

        let display = self.display.target();

        display.bounding_box()
            .draw_styled(
                &PrimitiveStyle::with_stroke(BinaryColor::On, 2),
                display,
            )
            .unwrap();

        let box_color = if self.on {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };
        Rectangle::new(Point::new(10, 10), Size::new(10, 10))
            .draw_styled(
                &PrimitiveStyle::with_fill(box_color),
                display,
            )
            .unwrap();

        self.display.flush();
    }
}
