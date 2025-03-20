use core::{fmt::Write, sync::atomic::Ordering};

use embassy_futures::yield_now;
use embassy_time::Timer;
use embedded_graphics::{image::{Image, ImageRaw}, mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::BinaryColor, prelude::{Dimensions, Point}, primitives::{PrimitiveStyle, StyledDrawable}, text::{Baseline, Text}, Drawable};
use ssd1306::mode::DisplayConfig;

use crate::{channel::INFO_REPORT_CHANNEL, event::KeyEvent, keycode::KeyCode, CONNECTION_TYPE};

pub(crate) enum PeripheralCallback {
    Layer(u8),
    KeyEvent(KeyEvent),
}

pub(crate) struct DisplayService<'a, I: embedded_hal::i2c::I2c> {
    interface: &'a mut I,
}

impl<'a, I: embedded_hal::i2c::I2c> DisplayService<'a, I> {
    pub(crate) fn new(interface: &'a mut I) -> Self {
        Self { interface }
    }

    pub(crate) async fn run(&mut self) {
        let interface = ::ssd1306::I2CDisplayInterface::new(&mut self.interface);
        let mut display = ::ssd1306::Ssd1306::new(
            interface,
            ::ssd1306::size::DisplaySize128x32,
            ::ssd1306::rotation::DisplayRotation::Rotate90,
        )
        .into_buffered_graphics_mode();
        display.init().unwrap();

        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("rust.raw"), 64);
        let im = Image::new(&raw, Point::new(0, 0));
        im.draw(&mut display).unwrap();

        display.flush().unwrap();

        let mut text = Text::with_baseline("", Point::new(0, 0), FONT, Baseline::Top);

        let mut layer = 0;
        loop {
            //TODO: update once every 100ms?

            let mut new_layer = layer;
            match INFO_REPORT_CHANNEL.receive().await {
                PeripheralCallback::Layer(l) => new_layer = l,
                PeripheralCallback::KeyEvent(key_event) => {},
            }
            yield_now().await;

            if new_layer != layer {
                layer = new_layer;
                let _ = text.bounding_box().draw_styled(&PrimitiveStyle::with_fill(BinaryColor::Off), &mut display);
                text.text = layer_label(layer);
                let _ = text.draw(&mut display);
            }

            let _ = display.flush();
            yield_now().await;
        }
    }
}

const FONT: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new().font(&FONT_6X10).text_color(BinaryColor::On).build();

const TEXT_BASE: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("BASE", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_MOD: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("MOD", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_SYM: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("SYM", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_NUM: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("NUM", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_ACC: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("ACC", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_COMM: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("COMM", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_GAME: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("GAME", Point::new(15, 4), FONT, Baseline::Middle);
const TEXT_NONE: Text<'static, MonoTextStyle<'static, BinaryColor>> = Text::with_baseline("NONE", Point::new(15, 4), FONT, Baseline::Middle);

fn layer_label(layer: u8) -> &'static str {
    match layer {
        0 => "BASE",
        1 => "MOD",
        2 => "SYM",
        3 => "NUM",
        4 => "ACC",
        5 => "COMM",
        6 => "GAME",
        7 => "GAME",
        _ => "NONE",
    }
}

//pub async fn run_display<'d, T: embassy_nrf::twim::Instance>(
//    twim: impl embassy_nrf::Peripheral<P = T> + 'd,
//    irqs: impl embassy_nrf::interrupt::typelevel::Binding<T::Interrupt, embassy_nrf::twim::InterruptHandler<T>> + 'd,
//    sda: impl embassy_nrf::Peripheral<P = impl embassy_nrf::gpio::Pin> + 'd,
//    scl: impl embassy_nrf::Peripheral<P = impl embassy_nrf::gpio::Pin> + 'd,
//    config: embassy_nrf::twim::Config,
//) {
//    // display stuff
//    use ::ssd1306::mode::DisplayConfig;
//    use ::core::fmt::Write;
//    //::rmk::run_display(&mut p.TWISPI0, Irqs, &mut p.P0_17, &mut p.P0_20, ::embassy_nrf::twim::Config::default()),
//    let mut twi = ::embassy_nrf::twim::Twim::new(twim, irqs, sda, scl, config);
//
//    let interface = ::ssd1306::I2CDisplayInterface::new(&mut twi);
//    let mut display = ::ssd1306::Ssd1306::new(interface, ::ssd1306::size::DisplaySize128x32, ::ssd1306::rotation::DisplayRotation::Rotate0).into_terminal_mode();
//    display.init().unwrap();
//    display.clear().unwrap();
//
//    for c in 97..123 {
//        let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//    }
//    for c in 65..91 {
//        let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//    }
//
//    //::core::mem::drop(twi);
//}
