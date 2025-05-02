use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use embedded_graphics::{image::{Image, ImageRaw}, mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, primitives::{PrimitiveStyle, Rectangle}, text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyleBuilder}};
use ssd1306::{mode::BufferedGraphicsModeAsync, prelude::*, Ssd1306Async};

use crate::{channel::OUTPUT_DEVICE_CHANNEL, hid_state::HidModifiers};

use super::{OutputDevice, OutputDeviceEvent};

static LAYER_NAMES: [&str; 8] = [
    "BASE",
    "NAV",
    "SYM",
    "NUM",
    "ACC",
    "COM",
    "GAME 1",
    "GAME 2",
];

pub struct DisplayDevice<'a> {
// pub struct DisplayDevice<DI: WriteOnlyDataCommand, SIZE: DisplaySize> {
//     display: Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
    display: Ssd1306Async<I2CInterface<Twim<'a, TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>,
    layer: u8,
    modifiers: HidModifiers,
}

impl<'a> DisplayDevice<'a> {
    pub fn new(display: Ssd1306Async<I2CInterface<Twim<'a, TWISPI0>>, DisplaySize128x32, BufferedGraphicsModeAsync<DisplaySize128x32>>) -> Self {
        Self {
            display,
            layer: 0,
            modifiers: HidModifiers::new(),
        }
    }

    pub async fn run(&mut self) {
        let mut sub = OUTPUT_DEVICE_CHANNEL.subscriber().unwrap();
        let mut layer = 0;
        let mut modifiers = HidModifiers::new();

        let character_style = MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18, BinaryColor::On);
        let character_smaller = MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_6X10, BinaryColor::On);
        let fill_style = PrimitiveStyle::with_fill(BinaryColor::On);
        let centered_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Center)
            .build();

        let layer_center = Point::new(
            character_style.measure_string("GAME 2", Point::zero(), Baseline::Alphabetic).bounding_box.center().x,
            self.display.bounding_box().center().y,
        );

        let raw = ImageRaw::<BinaryColor>::new(include_bytes!("./shift.raw"), 12);
        let im_shift = Image::with_center(&raw, self.display.bounding_box().center() + Point::new(0, 0));
        let raw = ImageRaw::<BinaryColor>::new(include_bytes!("./ctrl.raw"), 12);
        let im_ctrl = Image::with_center(&raw, self.display.bounding_box().center() + Point::new(15, 0));
        let raw = ImageRaw::<BinaryColor>::new(include_bytes!("./alt.raw"), 12);
        let im_alt = Image::with_center(&raw, self.display.bounding_box().center() + Point::new(30, 0));
        let raw = ImageRaw::<BinaryColor>::new(include_bytes!("./gui.raw"), 12);
        let im_gui = Image::with_center(&raw, self.display.bounding_box().center() + Point::new(45, 0));

        loop {
            loop {
                let event = sub.try_next_message_pure();
                if let Some(event) = event {
                    match event {
                        OutputDeviceEvent::LayerChange(l) => layer = l,
                        OutputDeviceEvent::Modifier(m) => modifiers = m,
                    }
                } else {
                    break;
                }
            }

            self.display.clear_buffer();

            if layer > 0 {
                Text::with_text_style(
                    LAYER_NAMES[layer as usize -1],
                    layer_center - Point::new(0, character_style.font.character_size.height as i32/3*2),
                    character_smaller,
                    centered_style,
                )
                    .draw(&mut self.display).unwrap();
            }

            if layer < LAYER_NAMES.len() as u8 -1 {
                Text::with_text_style(
                    LAYER_NAMES[layer as usize +1],
                    layer_center + Point::new(0, character_style.font.character_size.height as i32/3*2),
                    character_smaller,
                    centered_style,
                )
                    .draw(&mut self.display).unwrap();
            }

            for y in [0, 1, 3, 31, 30, 28] {
                for x in 0..127 {
                    Pixel(Point::new(x, y), BinaryColor::Off).draw(&mut self.display).unwrap()
                }
            }

            Text::with_text_style(
                LAYER_NAMES[layer as usize],
                layer_center,
                character_style,
                centered_style,
            )
                .draw(&mut self.display).unwrap();

            im_shift
                .translate(Point::new(0, (modifiers.left_shift() || modifiers.right_shift()) as i32*-4))
                .draw(&mut self.display).unwrap();
            if modifiers.left_shift() || modifiers.right_shift() {
                Rectangle::with_center(
                    self.display.bounding_box().center() + Point::new(0, 5),
                    Size::new(12, 2),
                )
                    .into_styled(fill_style)
                    .draw(&mut self.display).unwrap();
            }
            im_ctrl
                .translate(Point::new(0, (modifiers.left_ctrl() || modifiers.right_ctrl()) as i32*-4))
                .draw(&mut self.display).unwrap();
            if modifiers.left_ctrl() || modifiers.right_ctrl() {
                Rectangle::with_center(
                    self.display.bounding_box().center() + Point::new(15, 5),
                    Size::new(12, 2),
                )
                    .into_styled(fill_style)
                    .draw(&mut self.display).unwrap();
            }
            im_alt
                .translate(Point::new(0, (modifiers.left_alt() || modifiers.right_alt()) as i32*-4))
                .draw(&mut self.display).unwrap();
            if modifiers.left_alt() || modifiers.right_alt() {
                Rectangle::with_center(
                    self.display.bounding_box().center() + Point::new(30, 5),
                    Size::new(12, 2),
                )
                    .into_styled(fill_style)
                    .draw(&mut self.display).unwrap();
            }
            im_gui
                .translate(Point::new(0, (modifiers.left_gui() || modifiers.right_gui()) as i32*-4))
                .draw(&mut self.display).unwrap();
            if modifiers.left_gui() || modifiers.right_gui() {
                Rectangle::with_center(
                    self.display.bounding_box().center() + Point::new(45, 5),
                    Size::new(12, 2),
                )
                    .into_styled(fill_style)
                    .draw(&mut self.display).unwrap();
            }

            self.display.flush().await.unwrap();

            embassy_time::Timer::after_millis(1).await;
        }
    }
}

impl<'a> OutputDevice for DisplayDevice<'a> {
    async fn process_event(&mut self, e: OutputDeviceEvent) {
        match e {
            OutputDeviceEvent::LayerChange(l) => self.layer = l,
            OutputDeviceEvent::Modifier(m) => self.modifiers = m,
        }
    }
}
