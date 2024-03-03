use esp32_nimble::BLERemoteService;

use super::xboxone::GamepadPacketXboxOne;

/// GAMEPADS is a list of gamepads that are compatible with this library.
/// Add the name of the gamepad to the list to make it compatible, make sure that the gamepad is implemented.
// TODO: Improve method to check gamepad compatibility
const GAMEPAD_COMPATIBLE: [&str; 1] = ["XBOX WIRELESS CONTROLLER"];

// TODO: Improve method to check gamepad compatibility
pub fn check_gamepad_compatibility(device_name: &str) -> bool {
    GAMEPAD_COMPATIBLE.iter().any(|&name| device_name.to_lowercase().contains(&name.to_lowercase()))
}

#[derive(Debug)]
pub enum GamepadType {
    XboxOne,
}

/// responsible for handling all gamepad packets, from xboxone, ps4, etc.
pub enum GamepadPacket {
    XboxOne(GamepadPacketXboxOne),
}

/// Trait responsible for handling gamepad packets, parse characteristics and services to get the gamepad packet
pub trait GamepadPacketHandler<B, S, T> {
    fn buttons(raw_data: Vec<u8>) -> B;
    fn sticks(raw_data: Vec<u8>) -> S;
    fn trigger(raw_data: Vec<u8>) -> T;
    fn battery(raw_data: Vec<u8>) -> u8;
    /// Parse buttons fn, sticks fn and trigger fn to get the gamepad packet
    fn parse_packet(&self, svcs: Vec<&mut BLERemoteService>) -> Self;
}
