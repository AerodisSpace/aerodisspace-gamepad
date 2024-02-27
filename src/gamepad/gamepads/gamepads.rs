use super::xboxone::GamepadXboxOne;

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

/// GamepadDevice is an enum that is used to define the different gamepad devices
pub enum GamepadDevice<'a> {
    XboxOne(GamepadXboxOne<'a>),
}

/// GamepadHandle is a trait that is implemented by the gamepad devices
/// to handle the buttons, sticks, triggers, and battery.
/// Make sure to implement the GamepadHandle trait for each gamepad device.
pub trait GamepadHandle {
    fn connected(&mut self) -> bool;
    fn get_device_data(&mut self) -> anyhow::Result<()>;
    fn handle_buttons(&mut self) -> anyhow::Result<()>;
    fn handle_sticks(&mut self) -> anyhow::Result<()>;
    fn handle_triggers(&mut self) -> anyhow::Result<()>;
    fn handle_battery(&mut self) -> anyhow::Result<()>;
}
