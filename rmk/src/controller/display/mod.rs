use embassy_futures::select::{select, Either};
use embassy_nrf::twim::Twim;
use embedded_graphics::{image::{Image, ImageRaw}, mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::{Dimensions, Point, Transform, *}, primitives::{PrimitiveStyle, Rectangle}, text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyle, TextStyleBuilder}, Pixel};
use ssd1306::{mode::{BufferedGraphicsModeAsync, DisplayConfigAsync}, prelude::I2CInterface, size::DisplaySize128x32, Ssd1306Async};

use crate::{channel::{ControllerSub, CONTROLLER_CHANNEL}, event::ControllerEvent, keycode::ModifierCombination};

use super::{Controller, PollingController};

const LAYER_NAMES: [&str; 8] = [
    "BASE",
    "NAV",
    "SYM",
    "NUM",
    "ACC",
    "COM",
    "GAME",
    "GAME",
];

struct Graphics<'a> {
    character_style: MonoTextStyle<'a, BinaryColor>,
    character_smaller: MonoTextStyle<'a, BinaryColor>,
    fill_style: PrimitiveStyle<BinaryColor>,
    stroke_style: PrimitiveStyle<BinaryColor>,
    centered_style: TextStyle,
    layer_center: Point,
    raw_shift: ImageRaw<'a, BinaryColor>,
    raw_ctrl: ImageRaw<'a, BinaryColor>,
    raw_alt: ImageRaw<'a, BinaryColor>,
    raw_gui: ImageRaw<'a, BinaryColor>,
}

impl<'a> Graphics<'a> {
    fn new(bounding_box: Rectangle) -> Self {
        let character_style = MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18, BinaryColor::On);
        let character_smaller = MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_6X10, BinaryColor::On);
        let fill_style = PrimitiveStyle::with_fill(BinaryColor::On);
        let stroke_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let centered_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Center)
            .build();
        let layer_center = Point::new(
            character_style.measure_string("GAME 2", Point::zero(), Baseline::Alphabetic).bounding_box.center().x,
            bounding_box.center().y,
        );
        let raw_shift = ImageRaw::<BinaryColor>::new(include_bytes!("./shift.raw"), 12);
        let raw_ctrl = ImageRaw::<BinaryColor>::new(include_bytes!("./ctrl.raw"), 12);
        let raw_alt = ImageRaw::<BinaryColor>::new(include_bytes!("./alt.raw"), 12);
        let raw_gui = ImageRaw::<BinaryColor>::new(include_bytes!("./gui.raw"), 12);

        Self {
            character_style,
            character_smaller,
            fill_style,
            stroke_style,
            centered_style,
            layer_center,
            raw_shift,
            raw_ctrl,
            raw_alt,
            raw_gui,
        }
    }
}

pub(crate) struct DisplayController<'a> {
    sub: ControllerSub,
    init: bool,
    display: Ssd1306Async<I2CInterface<Twim<'a, embassy_nrf::peripherals::TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>,
    layer: u8,
    modifiers: ModifierCombination,
    battery: u8,
    graphics: Graphics<'a>,
}

impl<'a> DisplayController<'a> {
    pub fn new(display: Ssd1306Async<I2CInterface<Twim<'a, embassy_nrf::peripherals::TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>) -> Self {
        let bounding_box = display.bounding_box();
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            init: false,
            display,
            layer: 0,
            modifiers: ModifierCombination::new(),
            battery: 0,
            graphics: Graphics::new(bounding_box),
        }
    }

    pub fn unset_init(&mut self) {
        info!("display init unset");
        self.init = false;
    }

    async fn draw(&mut self) -> Result<(), <Ssd1306Async<I2CInterface<Twim<'a, embassy_nrf::peripherals::TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>> as DrawTarget>::Error> {
        if self.layer > 0 {
            Text::with_text_style(
                LAYER_NAMES[self.layer as usize -1],
                self.graphics.layer_center - Point::new(0, self.graphics.character_style.font.character_size.height as i32/3*2),
                self.graphics.character_smaller,
                self.graphics.centered_style,
            )
                .draw(&mut self.display)?;
        }

        if self.layer < LAYER_NAMES.len() as u8 -1 {
            Text::with_text_style(
                LAYER_NAMES[self.layer as usize +1],
                self.graphics.layer_center + Point::new(0, self.graphics.character_style.font.character_size.height as i32/3*2),
                self.graphics.character_smaller,
                self.graphics.centered_style,
            )
                .draw(&mut self.display)?;
        }

        Text::with_text_style(
            LAYER_NAMES[self.layer as usize],
            self.graphics.layer_center,
            self.graphics.character_style,
            self.graphics.centered_style,
        )
            .draw(&mut self.display)?;

        Image::with_center(&self.graphics.raw_shift, self.display.bounding_box().center() + Point::new(0, 0))
            .translate(Point::new(0, (self.modifiers.shift()) as i32*-4))
            .draw(&mut self.display)?;
        if self.modifiers.shift() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(0, 5),
                Size::new(12, 2),
            )
                .into_styled(self.graphics.fill_style)
                .draw(&mut self.display)?;
        }
        Image::with_center(&self.graphics.raw_ctrl, self.display.bounding_box().center() + Point::new(15, 0))
            .translate(Point::new(0, self.modifiers.ctrl() as i32*-4))
            .draw(&mut self.display)?;
        if self.modifiers.ctrl() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(15, 5),
                Size::new(12, 2),
            )
                .into_styled(self.graphics.fill_style)
                .draw(&mut self.display)?;
        }
        Image::with_center(&self.graphics.raw_alt, self.display.bounding_box().center() + Point::new(30, 0))
            .translate(Point::new(0, self.modifiers.alt() as i32*-4))
            .draw(&mut self.display)?;
        if self.modifiers.alt() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(30, 5),
                Size::new(12, 2),
            )
                .into_styled(self.graphics.fill_style)
                .draw(&mut self.display)?;
        }
        Image::with_center(&self.graphics.raw_gui, self.display.bounding_box().center() + Point::new(45, 0))
            .translate(Point::new(0, self.modifiers.gui() as i32*-4))
            .draw(&mut self.display)?;
        if self.modifiers.gui() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(45, 5),
                Size::new(12, 2),
            )
                .into_styled(self.graphics.fill_style)
                .draw(&mut self.display)?;
        }

        Rectangle::with_corners(Point::new(123, 0), Point::new(127, 31))
            .into_styled(self.graphics.stroke_style)
            .draw(&mut self.display)?;

        Rectangle::with_corners(Point::new(124, 32 - (self.battery as i32*32)/100), Point::new(126, 31))
            .into_styled(self.graphics.fill_style)
            .draw(&mut self.display)?;

        Ok(())
    }
}

impl<'a> Controller for DisplayController<'a> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Layer(layer) => {
                self.layer = layer;
                self.update().await;
            }
            ControllerEvent::Modifier(modifiers) => {
                self.modifiers = modifiers;
                self.update().await;
            }
            ControllerEvent::Battery(battery) => {
                self.battery = battery;
                self.update().await;
            }
            _ => (),
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'a> PollingController for DisplayController<'a> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_hz(24);

    async fn update(&mut self) {
        if !self.init {
            info!("display init");
            match select(self.display.init(), embassy_time::Timer::after_secs(1)).await {
                Either::First(r) => match r {
                    Ok(_) => self.init = true,
                    Err(_) => {
                        info!("display init failed");
                        return;
                    }
                }
                Either::Second(_) => {
                    info!("display init timed out");
                    return;
                }
            }
        }

        self.display.clear_buffer();

        if let Err(_) = self.draw().await {
            info!("draw failed");
            self.init = false;
        }

        if let Err(_) = self.display.flush().await {
            info!("flush failed");
            self.init = false;
        }
    }
}
