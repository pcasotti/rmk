use embassy_futures::yield_now;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::{Point, *}, primitives::{PrimitiveStyleBuilder, Rectangle}};
use embedded_hal::digital::OutputPin;
use ssd1306::{mode::DisplayConfig as _, prelude::DisplayRotation, size::DisplaySize128x32, I2CDisplayInterface, Ssd1306};

use crate::config::DisplayConfig;

pub struct DisplayService<'a, P: OutputPin> {
    pub enabled: bool,
    display_controller: &'a mut DisplayController<P>,
}

impl<'a, P: OutputPin> DisplayService<'a, P> {
    pub fn new(display_controller: &'a mut DisplayController<P>) -> Self {
        Self {
            enabled: false,
            display_controller,
        }
    }

    pub async fn run(&mut self) {
        loop {
            yield_now().await;
        }
    }
}

pub struct DisplayController<P: OutputPin> {
    scl: Option<P>,
    sda: Option<P>,
}

impl<P: OutputPin> DisplayController<P> {
    pub fn new(display_config: DisplayConfig<P>) -> Self {
        Self {
            scl: display_config.scl,
            sda: display_config.sda,
        }
    }
}

pub async fn run<I: embedded_hal::i2c::I2c>(i2c: I) {
    // let config = twim::Config::default();
    // let mut twi = Twim::new(p.TWISPI0, Irqs, p.P0_03, p.P0_04, config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Size::new(127, 31))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        yield_now().await;
    }
}
