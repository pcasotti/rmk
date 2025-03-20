use core::fmt::Write;

use embassy_futures::yield_now;
use ssd1306::mode::DisplayConfig;

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
            ::ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_terminal_mode();
        display.init().unwrap();
        display.clear().unwrap();

        loop {
            for c in 97..123 {
                let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
                yield_now().await;
            }
            for c in 65..91 {
                let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
                yield_now().await;
            }
        }
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
