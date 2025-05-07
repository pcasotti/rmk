use embedded_graphics::{pixelcolor::BinaryColor, prelude::{Dimensions, Point, Size}, primitives::{PrimitiveStyle, Rectangle, StyledDrawable}};
use embedded_hal_async::i2c::I2c;
use ssd1306::mode::DisplayConfigAsync;

use crate::{channel::{ControllerSub, CONTROLLER_CHANNEL}, event::ControllerEvent, keycode::ModifierCombination};

use super::{Controller, PollingController};

pub struct DisplayController<'a, I: I2c> {
    sub: ControllerSub<'a>,
    display: ssd1306::Ssd1306Async<ssd1306::prelude::I2CInterface<I>, ssd1306::size::DisplaySize128x32, ssd1306::mode::BufferedGraphicsModeAsync<ssd1306::size::DisplaySize128x32>>,
    modifiers: ModifierCombination,
}

impl<'a, I: I2c> DisplayController<'a, I> {
    pub async fn new(i2c: I) -> Self {
        let interface = ssd1306::I2CDisplayInterface::new(i2c);
        let mut display = ssd1306::Ssd1306Async::new(
            interface,
            ssd1306::size::DisplaySize128x32,
            ssd1306::prelude::DisplayRotation::Rotate0,
        )
            .into_buffered_graphics_mode();
        display.init().await.unwrap();
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            display,
            modifiers: ModifierCombination::new(),
        }
    }
}

impl<'a, I: I2c> Controller for DisplayController<'a, I> {
    type Event = ControllerEvent;

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'a, I: I2c> PollingController for DisplayController<'a, I> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_hz(60);

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Modifier(m) => self.modifiers = m,
            _ => (),
        }
    }

    async fn update(&mut self) {
        self.display.bounding_box()
            .draw_styled(
                &PrimitiveStyle::with_stroke(BinaryColor::On, 2),
                &mut self.display,
            )
            .unwrap();

        let box_color = if self.modifiers.shift() {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };
        Rectangle::new(Point::new(10, 10), Size::new(10, 10))
            .draw_styled(
                &PrimitiveStyle::with_fill(box_color),
                &mut self.display,
            )
            .unwrap();

        self.display.flush().await.unwrap();
    }
}
