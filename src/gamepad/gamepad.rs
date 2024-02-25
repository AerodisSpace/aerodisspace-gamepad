#![allow(unused)]

use esp32_nimble::BLEDevice;
use log::{info, warn};

pub struct Gamepad<'a> {
    _ble_device: &'a mut BLEDevice,
}

impl<'a> Gamepad<'a> {
    pub fn new() -> Gamepad<'a> {
        Gamepad {
            _ble_device: BLEDevice::take(),
        }
    }

    pub fn start(&mut self) {}

    pub async fn scan(&mut self) {
        let ble_scan = self
            ._ble_device
            .get_scan()
            .interval(100)
            .window(99)
            .on_result(|_scan, param| {
                // TODO: FIND GENERIC GAMEPADS DEVICES BY MANUFACTURER DATA, SERVICE UUID, APPEARANCE, ETC
                let xonestr = "C8:3F:26:A5:6C:00";
                if (param.addr().to_string() == xonestr) {
                    info!("XBOX ONE FOUND: {:#?}", param);
                } else {
                    warn!("Device found: {:#?}", param.addr());
                }
            });

        info!("Starting scan...");
        ble_scan.start(30000).await.unwrap();
        info!("End scan");
    }
}

// service_uuids
pub enum BleService {
    HumanInterfaceDevice = 0x1812,
}

pub enum BleAppearance {
    Gamepad = 0x3C4, // = 964
}
