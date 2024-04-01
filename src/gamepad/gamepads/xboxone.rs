#![allow(unused)]

// https://github.com/DJm00n/ControllersInfo/blob/master/xboxone/xboxone_model_1708_bluetooth_hid_report_descriptor.txt

use super::gamepads::GamepadPacketHandler;
use esp32_nimble::{uuid128, BLERemoteCharacteristic, BLERemoteService};
use esp_idf_hal::task::block_on;
use log::{error, info};

const HID_SVC_UUID: &str = "0x1812";
const HID_INFO_UUID: &str = "0x2a4a";
const HID_REPORT_MAP_UUID: &str = "0x2a4b";
const HID_CONTROL_POINT_UUID: &str = "0x2a4c";
const HID_REPORT_UUID: &str = "0x2a4d";

#[derive(Default, Debug)]
pub struct GamepadPacketXboxOne {
    /// Buttons
    pub buttons: Vec<ButtonsXboxOne>,
    /// Sticks (left, right) (x, y)
    pub sticks: [(u16, u16); 2],
    /// BRAKE, THROTTLE
    pub trigger: (u16, u16),
    pub battery: u8,
}

impl GamepadPacketHandler<Vec<ButtonsXboxOne>, [(u16, u16); 2], (u16, u16)> for GamepadPacketXboxOne {
    fn buttons(&mut self, raw_data: &[u8]) -> Vec<ButtonsXboxOne> {
        unimplemented!()
    }

    fn sticks(&mut self, raw_data: &[u8]) -> [(u16, u16); 2] {
        info!("{:?}", raw_data);
        if raw_data.len() >= 8 {
            let rx = u16::from_le_bytes([raw_data[0], raw_data[1]]);
            let ry = u16::from_le_bytes([raw_data[2], raw_data[3]]);
            let lx = u16::from_le_bytes([raw_data[4], raw_data[5]]);
            let ly = u16::from_le_bytes([raw_data[6], raw_data[7]]);
            [(lx, ly), (rx, ry)]
        } else {
            error!("Invalid data length: {:?}", raw_data.len());
            [(0, 0), (0, 0)]
        }
    }

    fn trigger(&mut self, raw_data: &[u8]) -> (u16, u16) {
        unimplemented!()
    }

    fn battery(&mut self, raw_data: &[u8]) -> u8 {
        unimplemented!()
    }

    fn parse_packet(&mut self, svcs: Vec<&mut BLERemoteService>) -> Self {
        let mut main_svc: Option<&mut BLERemoteService> = None;
        for svc in svcs {
            if svc.uuid().to_string() == HID_SVC_UUID {
                main_svc = Some(svc);
            }
        }
        block_on(async {
            // Main (buttons, sticks, triggers, etc.)
            if let Some(svc) = main_svc {
                for chr in svc.get_characteristics().await.unwrap() {
                    // REPORT
                    if chr.can_read() && chr.uuid().to_string() == HID_REPORT_UUID {
                        let raw_data = chr.read_value().await.unwrap();
                        self.sticks = self.sticks(&raw_data);
                        info!("{:?}", self.sticks);
                    }
                }
            }
        });
        // info!("{:?}", raw_data);

        Self::default()
    }
}

#[derive(Debug)]
pub enum ButtonsXboxOne {
    A,
    B,
    X,
    Y,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    Back,
    LeftStick,
    RightStick,
    RT,
    RB,
    LT,
    LB,
}
