use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use embedded_graphics::{image::{Image, ImageRaw}, pixelcolor::BinaryColor, prelude::*, primitives::{PrimitiveStyle, Rectangle}};
use ssd1306::{mode::BufferedGraphicsModeAsync, prelude::*, Ssd1306Async};

use crate::{channel::{ControllerSub, CONTROLLER_CHANNEL}, event::ControllerEvent, keycode::ModifierCombination};

use super::{Controller, PollingController};

pub struct Images<'a> {
    shift: ImageRaw<'a, BinaryColor>,
    ctrl: ImageRaw<'a, BinaryColor>,
    alt: ImageRaw<'a, BinaryColor>,
    gui: ImageRaw<'a, BinaryColor>,
}

pub struct DisplayController<'a> {
    display: Ssd1306Async<I2CInterface<Twim<'a, TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>,
    layer: u8,
    modifiers: ModifierCombination,
    sub: ControllerSub<'a>,
    images: Images<'a>,
    ok: bool,
}

impl<'a> DisplayController<'a> {
    pub async fn new(mut display: Ssd1306Async<I2CInterface<Twim<'a, TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>) -> Self {
        let shift = ImageRaw::<BinaryColor>::new(include_bytes!("./shift.raw"), 12);
        let ctrl = ImageRaw::<BinaryColor>::new(include_bytes!("./ctrl.raw"), 12);
        let alt = ImageRaw::<BinaryColor>::new(include_bytes!("./alt.raw"), 12);
        let gui = ImageRaw::<BinaryColor>::new(include_bytes!("./gui.raw"), 12);

        let ok = display.init().await.is_ok();

        Self {
            display,
            layer: 0,
            modifiers: ModifierCombination::new(),
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            images: Images {
                shift,
                ctrl,
                alt,
                gui
            },
            ok,
        }
    }
}

impl<'a> Controller for DisplayController<'a> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Layer(l) => self.layer = l,
            ControllerEvent::Modifier(m) => self.modifiers = m,
            _ => (),
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'a> PollingController for DisplayController<'a> {
    const INTERVAL: embassy_time::Duration = embassy_time::Duration::from_hz(30);

    async fn update(&mut self) {
        if !self.ok { return }

        self.display.clear_buffer();

        let im_shift = Image::with_center(&self.images.shift, self.display.bounding_box().center() + Point::new(0, 0));
        let im_ctrl = Image::with_center(&self.images.ctrl, self.display.bounding_box().center() + Point::new(15, 0));
        let im_alt = Image::with_center(&self.images.alt, self.display.bounding_box().center() + Point::new(30, 0));
        let im_gui = Image::with_center(&self.images.gui, self.display.bounding_box().center() + Point::new(45, 0));


        let fill_style = PrimitiveStyle::with_fill(BinaryColor::On);

        im_shift
            .translate(Point::new(0, self.modifiers.shift() as i32*-4))
            .draw(&mut self.display).unwrap();

        if self.modifiers.shift() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(0, 5),
                Size::new(12, 2),
            )
                .into_styled(fill_style)
                .draw(&mut self.display).unwrap();
        }
        im_ctrl
            .translate(Point::new(0, self.modifiers.ctrl() as i32*-4))
            .draw(&mut self.display).unwrap();
        if self.modifiers.ctrl() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(15, 5),
                Size::new(12, 2),
            )
                .into_styled(fill_style)
                .draw(&mut self.display).unwrap();
        }
        im_alt
            .translate(Point::new(0, self.modifiers.alt() as i32*-4))
            .draw(&mut self.display).unwrap();
        if self.modifiers.alt() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(30, 5),
                Size::new(12, 2),
            )
                .into_styled(fill_style)
                .draw(&mut self.display).unwrap();
        }
        im_gui
            .translate(Point::new(0, self.modifiers.gui() as i32*-4))
            .draw(&mut self.display).unwrap();
        if self.modifiers.gui() {
            Rectangle::with_center(
                self.display.bounding_box().center() + Point::new(45, 5),
                Size::new(12, 2),
            )
                .into_styled(fill_style)
                .draw(&mut self.display).unwrap();
        }

        self.display.flush().await.unwrap();
    }
}
