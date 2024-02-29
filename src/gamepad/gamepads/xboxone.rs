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
impl<'a> GamepadHandle for GamepadXboxOne<'a> {
    fn init(&mut self) -> anyhow::Result<GamepadInfo> {
        todo!()
    }

    fn connected(&mut self) -> bool {
        self._ble_client.connected()
    }
    fn get_device_data(&mut self) -> anyhow::Result<GamepadInfo> {
        let mut gamepad_info = GamepadInfo::default();
        block_on(async {
            // Get the generic access service
            let generic_access_service = match self
                ._ble_client
                .get_service(
                    BleUuid::from_uuid128_string("00001800-0000-1000-8000-00805F9B34FB").unwrap(),
                )
                .await
            {
                Ok(service) => service,
                Err(_) => return Err(anyhow::anyhow!("Failed to get service for generic access")),
            };

            // Device name characteristic
            let device_name_characteristic_uuid =
                match BleUuid::from_uuid128_string("00002A00-0000-1000-8000-00805F9B34FB") {
                    Ok(uuid) => uuid,
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse UUID for device name")),
                };
            match generic_access_service
                .get_characteristic(device_name_characteristic_uuid)
                .await
            {
                Ok(characteristic) => match characteristic.read_value().await {
                    Ok(value) => match String::from_utf8(value) {
                        Ok(device_name) => Some(device_name),
                        Err(_) => {
                            error!("Error reading device name");
                            None
                        }
                    },
                    Err(_) => todo!(),
                },
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to get characteristic for device name"
                    ))
                }
            };

            //================================================================================================
            // Device Information Service

            let device_information_uuid =
                match BleUuid::from_uuid128_string("0000180A-0000-1000-8000-00805F9B34FB") {
                    Ok(uuid) => uuid,
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse UUID")),
                };

            let device_information_service =
                match self._ble_client.get_service(device_information_uuid).await {
                    Ok(service) => service,
                    Err(_) => return Err(anyhow::anyhow!("Failed to get service")),
                };
            let manufacturers_characteristic_uuid =
                match BleUuid::from_uuid128_string("00002A29-0000-1000-8000-00805F9B34FB") {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        return Err(anyhow::anyhow!("Failed to parse UUID for manufacturers"))
                    }
                };
            let firmware_characteristic_uuid =
                match BleUuid::from_uuid128_string("00002A26-0000-1000-8000-00805F9B34FB") {
                    Ok(uuid) => uuid,
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse UUID for firmware")),
                };
            let serial_characteristic_uuid =
                match BleUuid::from_uuid128_string("00002A25-0000-1000-8000-00805F9B34FB") {
                    Ok(uuid) => uuid,
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse UUID for serial")),
                };

            // TODO: Get the characteristics for the device information service and parse the values
            //================================================================================================
            Ok(gamepad_info)
        })
    }

    fn get_service_and_characteristic(&mut self) -> anyhow::Result<()> {
        info!("TEST data");
        let mut service: Option<&mut BLERemoteService> = None;
        block_on(async {
            service = Some(
                self._ble_client
                    .get_service(
                        BleUuid::from_uuid128_string(SERVICE_GAMEPAD_XBOX_ONE_1780).unwrap(),
                    )
                    .await
                    .unwrap(),
            );
        });

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

pub struct XboxOneGamepadPacket {
    pub button: u16,
    pub rt_trigger: u16,
    pub lt_trigger: u16,
    pub r_stick: (i16, i16),
    pub l_stick: (i16, i16),
    pub battery: u8,
}
