#![allow(unused)]

use esp32_nimble::{uuid128, BLERemoteCharacteristic, BLERemoteService};
use esp_idf_hal::task::block_on;
use log::info;

use super::gamepads::GamepadPacketHandler;

#[derive(Default, Debug)]
pub struct GamepadPacketXboxOne {
    /// Buttons
    pub buttons: Vec<ButtonsXboxOne>,
    /// Sticks (left, right) (x, y)
    pub sticks: [(i16, i16); 2],
    /// BRAKE, THROTTLE
    pub trigger: (u16, u16),
    pub battery: u8,
}

impl GamepadPacketHandler<Vec<ButtonsXboxOne>, [(i16, i16); 2], (u16, u16)> for GamepadPacketXboxOne {
    fn buttons(raw_data: Vec<u8>) -> Vec<ButtonsXboxOne> {
        unimplemented!()
    }

    fn sticks(raw_data: Vec<u8>) -> [(i16, i16); 2] {
        unimplemented!()
    }

    fn trigger(raw_data: Vec<u8>) -> (u16, u16) {
        unimplemented!()
    }

    fn battery(raw_data: Vec<u8>) -> u8 {
        unimplemented!()
    }

    fn parse_packet(&self, svcs: Vec<&mut BLERemoteService>) -> Self {
        let mut main_svc: Option<&mut BLERemoteService> = None;
        let mut battery_svc: Option<&mut BLERemoteService> = None;
        for svc in svcs {
            if svc.uuid() == uuid128!("00000001-5F60-4C4F-9C83-A7953298D40D") {
                main_svc = Some(svc);
            } else if svc.uuid() == uuid128!("00000002-5F60-4C4F-9C83-A7953298D40D") {
                battery_svc = Some(svc);
            }
        }

        block_on(async {
            // Main (buttons, sticks, triggers, etc.)
            if let Some(svc) = main_svc {
                for chr in svc.get_characteristics().await.unwrap() {
                    if chr.uuid() == uuid128!("00000002-5F60-4C4F-9C83-A7953298D40D") {
                        if chr.can_read() {
                            let raw_data = chr.read_value().await.unwrap();
                            info!("data giga:\n {:#?}", raw_data);
                        }
                    }
                }
            }
            // Battery
            if let Some(svc) = battery_svc {
                for chr in svc.get_characteristics().await.unwrap() {}
            }
        });

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
