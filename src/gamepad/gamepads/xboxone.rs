use super::gamepads::{GamepadHandle, GamepadInfo};
use esp32_nimble::{utilities::BleUuid, BLEAdvertisedDevice, BLEClient, BLERemoteService};
use esp_idf_hal::task::block_on;
use log::{error, info};

const SERVICE_GAMEPAD_XBOX_ONE_1780: &'static str = "00000001-5F60-4C4F-9C83-A7953298D40D";
const CHARACTERISTIC_GAMEPAD_XBOX_ONE_1780: [&'static str; 3] = [
    "00000002-5F60-4C4F-9C83-A7953298D40D",
    "00000003-5F60-4C4F-9C83-A7953298D40D",
    "00000004-5F60-4C4F-9C83-A7953298D40D",
];

pub struct GamepadXboxOne<'a> {
    pub(crate) _ble_device: &'a BLEAdvertisedDevice,
    pub(crate) _ble_client: &'a mut BLEClient,
}

// TODO: IMPLEMENT GAMEPADHANDLE FOR XBOXONE
// SERVICES AND CHARACTERISTICS
impl<'a> GamepadHandle<GamepadXboxOnePacket> for GamepadXboxOne<'a> {
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

    fn get_packet(&mut self) -> anyhow::Result<GamepadXboxOnePacket> {
        info!("TEST packet");
        let mut packet = GamepadXboxOnePacket::default();
        Ok(packet)
    }
}

#[derive(Default)]
pub struct GamepadXboxOnePacket {
    pub button: u16,
    pub rt_trigger: u16,
    pub lt_trigger: u16,
    pub r_stick: (i16, i16),
    pub l_stick: (i16, i16),
    pub battery: u8,
}
