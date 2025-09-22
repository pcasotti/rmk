use core::cell::RefCell;

use usbd_hid::descriptor::generator_prelude::*;
use embassy_time::Timer;
use embassy_usb::{class::hid::HidReaderWriter, driver::Driver};
use rmk_types::{action::KeyAction, protocol::rmk_rpc::{Endpoint, GetActiveLayer, GetKeyAction, SetKeyAction}};
use serde::Serialize;
use usbd_hid::descriptor::AsInputReport;

use crate::{event::{KeyPos, KeyboardEventPos}, hid::{HidError, HidReaderTrait, HidWriterTrait}, keymap::KeyMap, state::{ConnectionState, CONNECTION_STATE}};

macro_rules! link_handlers {
    (
        $($endpoint:ty, $handler:ident;)*
    ) => {
        async fn handle(&mut self, data: &[u8; 32]) -> [u8; 32] {
            let mut buf = [0; 32];
            let key = data[0];
            buf[0] = key;
            match key {
                $(
                    <$endpoint as Endpoint>::KEY => {
                        let request = postcard::from_bytes::<<$endpoint as Endpoint>::Request>(&data[1..]).unwrap();
                        let response = self.$handler(request).await;
                        postcard::to_slice(&response, &mut buf[1..]);
                    }
                )*
                _ => {
                    info!("Unknown cmd: {:?}", data);
                }
            }
            buf
        }
    };
}

pub(crate) trait Dispatcher {
    async fn handle(&mut self, data: &[u8; 32]) -> [u8; 32];
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = 0xFF70, usage = 0x71) = {
        input=input;
        output=output;
    }
)]
pub struct RmkRpcReport {
    pub input: [u8; 32],
    pub output: [u8; 32],
}

pub(crate) struct RmkRpcService<
    'a,
    RW: HidWriterTrait<ReportType = RmkRpcReport> + HidReaderTrait<ReportType = RmkRpcReport>,
    const ROW: usize,
    const COL: usize,
    const NUM_LAYER: usize,
    const NUM_ENCODER: usize,
> {
    // Reference of keymap, for updating
    keymap: &'a RefCell<KeyMap<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>>,

    // Usb vial hid reader writer
    pub(crate) reader_writer: RW,
}

impl<
    'a,
    RW: HidWriterTrait<ReportType = RmkRpcReport> + HidReaderTrait<ReportType = RmkRpcReport>,
    const ROW: usize,
    const COL: usize,
    const NUM_LAYER: usize,
    const NUM_ENCODER: usize,
> Dispatcher for RmkRpcService<'a, RW, ROW, COL, NUM_LAYER, NUM_ENCODER> {
    link_handlers! {
        GetKeyAction, get_key_action_handler;
        SetKeyAction, set_key_action_handler;
        GetActiveLayer, get_active_layer_handler;
    }
}

impl<
    'a,
    RW: HidWriterTrait<ReportType = RmkRpcReport> + HidReaderTrait<ReportType = RmkRpcReport>,
    const ROW: usize,
    const COL: usize,
    const NUM_LAYER: usize,
    const NUM_ENCODER: usize,
> RmkRpcService<'a, RW, ROW, COL, NUM_LAYER, NUM_ENCODER>
{
    pub(crate) fn new(
        keymap: &'a RefCell<KeyMap<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
        reader_writer: RW,
    ) -> Self {
        Self {
            keymap,
            reader_writer,
        }
    }

    pub(crate) async fn run(&mut self) {
        loop {
            match self.process().await {
                Ok(_) => continue,
                Err(e) => {
                    if ConnectionState::Disconnected == ConnectionState::from(&CONNECTION_STATE) {
                        Timer::after_millis(1000).await;
                    } else {
                        error!("Process rmk rpc error: {:?}", e);
                        Timer::after_millis(10000).await;
                    }
                }
            }
        }
    }

    pub(crate) async fn process(&mut self) -> Result<(), HidError> {
        let mut report = self.reader_writer.read_report().await?;
        debug!("processing rmk rpc report: {:?}", report.output);
        report.input = self.handle(&report.output).await;
        self.reader_writer.write_report(report).await?;

        Ok(())
    }

    async fn get_key_action_handler(&mut self, request: <GetKeyAction as Endpoint>::Request) -> <GetKeyAction as Endpoint>::Response {
        debug!("rpc request: {:?}", request);
        let pos = KeyboardEventPos::Key(KeyPos {
            row: request.row,
            col: request.col,
        });
        let response = Ok(self.keymap.borrow().get_action_at(pos, request.layer as usize));
        debug!("rpc response: {:?}", response);
        response
    }

    async fn set_key_action_handler(&mut self, request: <SetKeyAction as Endpoint>::Request) -> <SetKeyAction as Endpoint>::Response {
        let pos = KeyboardEventPos::Key(KeyPos {
            row: request.0.row,
            col: request.0.col,
        });
        self.keymap.borrow_mut().set_action_at(pos, request.0.layer as usize, request.1);
        Ok(())
    }

    async fn get_active_layer_handler(&mut self, request: <GetActiveLayer as Endpoint>::Request) -> <GetActiveLayer as Endpoint>::Response {
        self.keymap.borrow().get_activated_layer()
    }
}

pub struct UsbRmkRpcReaderWriter<'a, 'd, D: Driver<'d>> {
    pub(crate) reader_writer: &'a mut HidReaderWriter<'d, D, 32, 32>,
}

impl<'a, 'd, D: Driver<'d>> UsbRmkRpcReaderWriter<'a, 'd, D> {
    pub(crate) fn new(reader_writer: &'a mut HidReaderWriter<'d, D, 32, 32>) -> Self {
        Self { reader_writer: reader_writer }
    }
}

impl<'d, D: Driver<'d>> HidWriterTrait for UsbRmkRpcReaderWriter<'_, 'd, D> {
    type ReportType = RmkRpcReport;

    async fn write_report(&mut self, report: Self::ReportType) -> Result<usize, HidError> {
        self.reader_writer
            .write_serialize(&report)
            .await
            .map_err(HidError::UsbEndpointError)?;
        Ok(32)
    }
}

impl<'d, D: Driver<'d>> HidReaderTrait for UsbRmkRpcReaderWriter<'_, 'd, D> {
    type ReportType = RmkRpcReport;

    async fn read_report(&mut self) -> Result<Self::ReportType, HidError> {
        let mut read_report = RmkRpcReport {
            input: [0; 32],
            output: [0; 32],
        };
        self.reader_writer
            .read(&mut read_report.output)
            .await
            .map_err(HidError::UsbReadError)?;

        Ok(read_report)
    }
}
