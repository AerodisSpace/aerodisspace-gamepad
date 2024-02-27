use esp32_nimble::{BLEAdvertisedDevice, BLEClient};
use log::info;

use super::gamepads::GamepadHandle;

pub const UUID_SERVICE_HID_XBOX_ONE_1780: &'static str = "00000001-5F60-4C4F-9C83-A7953298D40D";

pub struct GamepadXboxOne<'a> {
    pub(crate) _ble_device: &'a BLEAdvertisedDevice,
    pub(crate) _ble_client: &'a BLEClient,
}

// TODO: IMPLEMENT GAMEPADHANDLE FOR XBOXONE
// SERVICES AND CHARACTERISTICS
impl<'a> GamepadHandle for GamepadXboxOne<'a> {
    fn connected(&mut self) -> bool {
        info!("TEST connected");
        self._ble_client.connected()
    }
    fn get_device_data(&mut self) -> anyhow::Result<()> {
        info!("TEST data");
        Ok(())
    }

    fn handle_buttons(&mut self) -> anyhow::Result<()> {
        info!("TEST buttons");
        Ok(())
    }

    fn handle_sticks(&mut self) -> anyhow::Result<()> {
        info!("TEST sticks");
        Ok(())
    }

    fn handle_triggers(&mut self) -> anyhow::Result<()> {
        info!("TEST trigger");
        Ok(())
    }

    fn handle_battery(&mut self) -> anyhow::Result<()> {
        info!("TEST battery");
        Ok(())
    }
}
