#![allow(unused)]

// https://github.com/DJm00n/ControllersInfo/blob/master/xboxone/xboxone_model_1708_bluetooth_hid_report_descriptor.txt

use std::sync::{Arc, Mutex};

use super::super::gamepads::GamepadPacketHandler;
use esp32_nimble::{uuid128, BLERemoteCharacteristic, BLERemoteService};
use esp_idf_hal::task::block_on;
use log::{error, info, warn};

const HID_SVC_UUID: &str = "0x1812";
const HID_INFO_UUID: &str = "0x2a4a";
const HID_REPORT_MAP_UUID: &str = "0x2a4b";
const HID_CONTROL_POINT_UUID: &str = "0x2a4c";
const HID_REPORT_UUID: &str = "0x2a4d";
const HID_DESCRIPTOR: [u8; 258] = [
    0x05, 0x01, 0x09, 0x05, 0xA1, 0x01, 0x85, 0x01, 0x09, 0x01, 0xA1, 0x00, 0x09, 0x30, 0x09, 0x31, 0x15, 0x00, 0x27,
    0xFF, 0xFF, 0x00, 0x00, 0x95, 0x02, 0x75, 0x10, 0x81, 0x02, 0xC0, 0x09, 0x01, 0xA1, 0x00, 0x09, 0x32, 0x09, 0x35,
    0x15, 0x00, 0x27, 0xFF, 0xFF, 0x00, 0x00, 0x95, 0x02, 0x75, 0x10, 0x81, 0x02, 0xC0, 0x05, 0x02, 0x09, 0xC5, 0x15,
    0x00, 0x26, 0xFF, 0x03, 0x95, 0x01, 0x75, 0x0A, 0x81, 0x02, 0x15, 0x00, 0x25, 0x00, 0x75, 0x06, 0x95, 0x01, 0x81,
    0x03, 0x05, 0x01, 0x09, 0x39, 0x15, 0x01, 0x25, 0x08, 0x35, 0x00, 0x46, 0x3B, 0x01, 0x66, 0x14, 0x00, 0x75, 0x04,
    0x95, 0x01, 0x81, 0x42, 0x75, 0x04, 0x95, 0x01, 0x15, 0x00, 0x25, 0x00, 0x35, 0x00, 0x45, 0x00, 0x65, 0x00, 0x81,
    0x03, 0x05, 0x09, 0x19, 0x01, 0x29, 0x0F, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x0F, 0x81, 0x02, 0x15, 0x00,
    0x25, 0x00, 0x75, 0x01, 0x95, 0x01, 0x81, 0x03, 0x05, 0x0C, 0x0A, 0xB2, 0x00, 0x15, 0x00, 0x25, 0x01, 0x95, 0x01,
    0x75, 0x01, 0x81, 0x02, 0x15, 0x00, 0x25, 0x00, 0x75, 0x07, 0x95, 0x01, 0x81, 0x03, 0x05, 0x0F, 0x09, 0x21, 0x85,
    0x03, 0xA1, 0x02, 0x09, 0x97, 0x15, 0x00, 0x25, 0x01, 0x75, 0x04, 0x95, 0x01, 0x91, 0x02, 0x15, 0x00, 0x25, 0x00,
    0x75, 0x04, 0x95, 0x01, 0x91, 0x03, 0x09, 0x70, 0x15, 0x00, 0x25, 0x64, 0x75, 0x08, 0x95, 0x04, 0x91, 0x02, 0x09,
    0x50, 0x66, 0x01, 0x10, 0x55, 0x0E, 0x15, 0x00, 0x26, 0xFF, 0x00, 0x75, 0x08, 0x95, 0x01, 0x91, 0x02, 0x09, 0xA7,
    0x15, 0x00, 0x26, 0xFF, 0x00, 0x75, 0x08, 0x95, 0x01, 0x91, 0x02, 0x65, 0x00, 0x55, 0x00, 0x09, 0x7C, 0x15, 0x00,
    0x26, 0xFF, 0x00, 0x75, 0x08, 0x95, 0x01, 0x91, 0x02, 0xC0, 0xC0,
];

#[derive(Debug, Default, Clone)]
pub struct GamepadXboxOne {
    pub debug: bool,
    /// Buttons
    pub buttons: ButtonsXboxOne,
    /// Sticks (left, right) (x, y)
    pub sticks: [(u16, u16); 2],
    /// BRAKE, THROTTLE
    pub trigger: (u8, u8),
    pub battery: u8,
}

impl GamepadPacketHandler for GamepadXboxOne {
    fn set_buttons(&mut self, raw_data: &[u8]) {
        self.buttons = ButtonsXboxOne::parse_bytes(raw_data);
    }

    fn set_sticks(&mut self, raw_data: &[u8]) {
        if raw_data.len() >= 8 {
            let rx = u16::from_le_bytes([raw_data[0], raw_data[1]]);
            let ry = u16::from_le_bytes([raw_data[2], raw_data[3]]);
            let lx = u16::from_le_bytes([raw_data[4], raw_data[5]]);
            let ly = u16::from_le_bytes([raw_data[6], raw_data[7]]);

            self.sticks = [(lx, ly), (rx, ry)];
        } else {
            error!("Invalid data length: {:?}", raw_data.len());
            self.sticks = [(0, 0), (0, 0)];
        }
    }

    fn set_trigger(&mut self, raw_data: &[u8]) {
        // LT Trigger 8 idx | LT Button 9 idx
        self.trigger.0 = raw_data[8];
        // RT Trigger 10 idx | RT Button 11 idx
        self.trigger.1 = raw_data[10];
    }

    fn set_battery(&mut self, raw_data: &[u8]) {
        self.battery = 0;
    }

    fn setup(self_arc: Arc<Mutex<Self>>, svcs: Vec<&mut BLERemoteService>) {
        self_arc.lock().unwrap().set_svcs(self_arc.clone(), svcs);
    }

    fn set_svcs(&mut self, self_arc: Arc<Mutex<Self>>, mut svcs: Vec<&mut BLERemoteService>) {
        let mut main_svc: Option<&mut BLERemoteService> = None;
        for svc in svcs {
            if svc.uuid().to_string() == HID_SVC_UUID {
                main_svc = Some(svc);
            }
        }
        block_on(async {
            // Main (buttons, sticks, triggers, etc.)
            if let Some(svc) = main_svc.as_mut() {
                for chr in svc.get_characteristics().await.unwrap() {
                    // REPORT
                    if chr.can_read() && chr.can_notify() && chr.uuid().to_string() == HID_REPORT_UUID {
                        let _self_arc_clone = self_arc.clone();
                        chr.on_notify(move |raw_data| {
                            let mut _self_lock = _self_arc_clone.lock().unwrap();
                            if _self_lock.debug {
                                warn!("{:?}", raw_data);
                            }
                            _self_lock.set_sticks(raw_data);
                            _self_lock.set_buttons(raw_data);
                            _self_lock.set_trigger(raw_data);
                            _self_lock.set_battery(raw_data);
                            if let Some(misc_btn) = _self_lock.buttons.misc {
                                match misc_btn {
                                    ButtonMiscXboxOne::MainXboxButton => _self_lock.debug = !_self_lock.debug,
                                    ButtonMiscXboxOne::Start => {}
                                    ButtonMiscXboxOne::View => {}
                                    ButtonMiscXboxOne::RightStick => {}
                                    ButtonMiscXboxOne::LeftStick => {}
                                }
                            }
                        });
                        chr.subscribe_notify(true);
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ButtonMiscXboxOne {
    MainXboxButton = 16,
    Start = 8,
    View = 4,
    RightStick = 64,
    LeftStick = 32,
}

impl ButtonMiscXboxOne {
    fn from_u8(raw_data: u8) -> Option<Self> {
        match raw_data {
            16 => Some(Self::MainXboxButton),
            8 => Some(Self::Start),
            4 => Some(Self::View),
            64 => Some(Self::RightStick),
            32 => Some(Self::LeftStick),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ButtonDpadXboxOne {
    Up = 1,
    Down = 5,
    Left = 7,
    Right = 3,
}

impl ButtonDpadXboxOne {
    fn from_u8(raw_data: u8) -> Option<Self> {
        match raw_data {
            1 => Some(Self::Up),
            5 => Some(Self::Down),
            7 => Some(Self::Left),
            3 => Some(Self::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ButtonCommonXboxOne {
    A = 1,
    B = 2,
    X = 8,
    Y = 16,
    LB = 64,
    RB = 128,
}
impl ButtonCommonXboxOne {
    fn from_u8(raw_data: u8) -> Option<Self> {
        match raw_data {
            1 => Some(Self::A),
            2 => Some(Self::B),
            8 => Some(Self::X),
            16 => Some(Self::Y),
            64 => Some(Self::LB),
            128 => Some(Self::RB),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ButtonTriggerXboxOne {
    LT,
    RT,
}

impl ButtonTriggerXboxOne {
    fn from_u8(raw_data: &[u8]) -> Option<Self> {
        if (raw_data[8] == 255) && (raw_data[9] == 3) {
            Some(Self::LT)
        } else if (raw_data[10] == 255) && (raw_data[11] == 3) {
            Some(Self::RT)
        } else {
            None
        }
    }
}

// TODO: CHANGE TO STRUCT WITH OPTION FIELD
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct ButtonsXboxOne {
    pub common: Option<ButtonCommonXboxOne>,
    pub trigger: Option<ButtonTriggerXboxOne>,
    pub misc: Option<ButtonMiscXboxOne>,
    pub dpad: Option<ButtonDpadXboxOne>,
}

impl ButtonsXboxOne {
    pub fn parse_bytes(raw_data: &[u8]) -> Self {
        //idx 8+
        let mut buttons = Self {
            common: None,
            trigger: None,
            misc: None,
            dpad: None,
        };
        buttons.common = ButtonCommonXboxOne::from_u8(raw_data[13]);
        buttons.dpad = ButtonDpadXboxOne::from_u8(raw_data[12]);
        buttons.trigger = ButtonTriggerXboxOne::from_u8(raw_data);
        buttons.misc = ButtonMiscXboxOne::from_u8(raw_data[14]);

        buttons
    }
}
