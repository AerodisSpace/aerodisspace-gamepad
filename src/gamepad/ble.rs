#![allow(unused)]

// service_uuids
pub enum BleService {
    HumanInterfaceDevice = 0x1812, // HID
}

pub enum BleAppearance {
    Gamepad = 0x3C4, // = 964
}

pub enum BleCharacteristic {
    HidInformation = 0x2a4a,
    HidControlPoint = 0x2a4c,
    ReportMap = 0x2a4b,
    Report = 0x2a4d,
}
