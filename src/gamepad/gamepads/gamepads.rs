use std::sync::{Arc, Mutex};

use esp32_nimble::BLERemoteService;

use super::xboxone::xboxone::GamepadXboxOne;

/// GAMEPADS is a list of gamepads that are compatible with this library.
/// Add the name of the gamepad to the list to make it compatible, make sure that the gamepad is implemented.
// TODO: Improve method to check gamepad compatibility
const GAMEPAD_COMPATIBLE: [&str; 1] = ["XBOX WIRELESS CONTROLLER"];

// TODO: Improve method to check gamepad compatibility
pub fn check_gamepad_compatibility(device_name: &str) -> bool {
    GAMEPAD_COMPATIBLE
        .iter()
        .any(|&name| device_name.to_lowercase().contains(&name.to_lowercase()))
}

#[derive(Debug)]
pub enum GamepadType {
    XboxOne,
}

/// responsible for handling all gamepad packets, from xboxone, ps4, etc.
pub enum GamepadHandler {
    XboxOne(Arc<Mutex<GamepadXboxOne>>),
}

/// Trait responsible for handling gamepad packets, parse characteristics and services to get the gamepad packet
pub trait GamepadPacketHandler {
    fn setup(self_arc: Arc<Mutex<GamepadXboxOne>>, svcs: Vec<&mut BLERemoteService>);
    fn set_svcs(&mut self, self_arc: Arc<Mutex<Self>>, svcs: Vec<&mut BLERemoteService>);
    fn set_buttons(&mut self, raw_data: &[u8]);
    fn set_sticks(&mut self, raw_data: &[u8]);
    fn set_trigger(&mut self, raw_data: &[u8]);
    fn set_battery(&mut self, raw_data: &[u8]);
}
