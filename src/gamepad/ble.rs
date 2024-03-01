use anyhow::{bail, Result};
use esp32_nimble::{utilities::BleUuid, BLEAdvertisedDevice, BLEClient, BLEDevice};
use log::{error, info, warn};

use super::gamepads::{
    gamepads::{GamepadDevice, GamepadInfo},
    xboxone::GamepadXboxOne,
};
use crate::gamepad::gamepads::gamepads::check_gamepad_compatibility;

pub struct BLEGamepad<'a> {
    pub connected: bool,
    _gamepad_device: Option<BLEAdvertisedDevice>,
    _ble_device: &'a mut BLEDevice,
    _ble_client: BLEClient,
}

impl<'a> BLEGamepad<'a> {
    pub fn new() -> BLEGamepad<'a> {
        BLEGamepad {
            connected: false,
            _gamepad_device: None,
            _ble_device: BLEDevice::take(),
            _ble_client: BLEClient::new(),
        }
    }

    pub async fn scan_and_connect(&mut self) -> Result<()> {
        self.scan().await?;
        self.connect().await?;
        self.client_handles()?;
        Ok(())
    }

    async fn scan(&mut self) -> Result<()> {
        info!("Scanning Gamepads...");
        let ble_scan = self._ble_device.get_scan();
        match ble_scan
            .active_scan(true)
            .interval(150)
            .window(150)
            .find_device(10000, |_device| check_gamepad_compatibility(&_device.name().to_string()))
            .await
        {
            Ok(device) => {
                if let Some(device) = device {
                    info!("Gamepad found: {:?}", device.name());
                    self._gamepad_device = Some(device);
                } else {
                    warn!("No gamepad found");
                }
            }
            Err(_) => {
                error!("Error scanning for gamepads");
                bail!("Error scanning for gamepads");
            }
        }

        Ok(())
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to gamepad...");
        if self.connected {
            info!("Already connected to gamepad");
            return Ok(());
        }
        if let Some(device) = &self._gamepad_device {
            let _ = self._ble_client.connect(device.addr()).await;
            self.connected = self._ble_client.connected();
        }

        Ok(())
    }

    fn client_handles(&mut self) -> Result<()> {
        self._ble_client.on_connect(|client| {
            client.update_conn_params(120, 120, 0, 60).unwrap();
            info!("Connected to gamepad");
        });

        self._ble_client.on_disconnect(|_| {
            info!("Disconnected from gamepad");
        });

        self._ble_client.on_passkey_request(|| 123456);

        self._ble_client.on_confirm_pin(|pin| {
            info!("Pin: {}", pin);
            true
        });

        Ok(())
    }

    pub fn connected(&mut self) -> bool {
        self._ble_client.connected()
    }

    pub fn get_gamepad(&mut self) -> Result<GamepadDevice> {
        if self._ble_client.connected() {
            if let Some(device) = &self._gamepad_device {
                if device.name().to_string().to_lowercase().contains("xbox") {
                    Ok(GamepadDevice::XboxOne(GamepadXboxOne {
                        _ble_device: &device,
                        _ble_client: &mut self._ble_client,
                    }))
                } else {
                    error!("Gamepad not supported");
                    bail!("Gamepad not supported");
                }
            } else {
                error!("No gamepad found");
                bail!("No gamepad found");
            }
        } else {
            error!("No gamepad found");
            bail!("No gamepad found");
        }
    }

    pub async fn get_device_data(&mut self) -> anyhow::Result<GamepadInfo> {
        let mut gamepad_info = GamepadInfo::default();

        // Get the generic access service
        match self
            ._ble_client
            .get_service(BleUuid::from_uuid128_string(GENERIC_ACCESS_SERVICE_UUID).unwrap())
            .await
        {
            Ok(service) => {
                // Device name characteristic
                match BleUuid::from_uuid128_string(GENERIC_ACCESS_SERVICE_CHARACTERISTIC_DEVICE_NAME_UUID) {
                    Ok(uuid) => match service.get_characteristic(uuid).await {
                        Ok(ch) => match ch.read_value().await {
                            Ok(val) => match String::from_utf8(val) {
                                Ok(data) => gamepad_info.device_name = Some(data),
                                Err(_) => {}
                            },
                            Err(_) => {}
                        },
                        Err(_) => {}
                    },
                    Err(_) => return Err(anyhow::anyhow!("Failed to parse UUID for device name")),
                };
            }
            Err(_) => return Err(anyhow::anyhow!("Failed to get service for generic access")),
        };

        //================================================================================================
        // Device Information Service

        match self
            ._ble_client
            .get_service(BleUuid::from_uuid128_string(DEVICE_INFORMATION_SERVICE_UUID).unwrap())
            .await
        {
            Ok(service) => {
                // Device manufactures characteristic
                match service
                    .get_characteristic(BleUuid::from_uuid128_string(DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_MANUFACTURER_NAME_UUID).unwrap())
                    .await
                {
                    Ok(ch) => match ch.read_value().await {
                        Ok(val) => match String::from_utf8(val) {
                            Ok(data) => gamepad_info.device_manufacturer = Some(data),
                            Err(_) => {}
                        },
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
                // Device firmware characteristic
                match service
                    .get_characteristic(BleUuid::from_uuid128_string(DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_FIRMWARE_REVISION_UUID).unwrap())
                    .await
                {
                    Ok(ch) => match ch.read_value().await {
                        Ok(val) => match String::from_utf8(val) {
                            Ok(data) => gamepad_info.device_firmware = Some(data),
                            Err(_) => {}
                        },
                        Err(_) => {}
                    },
                    Err(_) => {}
                }

                // Device Serial characteristic
                match service
                    .get_characteristic(BleUuid::from_uuid128_string(DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_SERIAL_NUMBER_UUID).unwrap())
                    .await
                {
                    Ok(ch) => match ch.read_value().await {
                        Ok(val) => match String::from_utf8(val) {
                            Ok(data) => gamepad_info.device_serial = Some(data),
                            Err(_) => {}
                        },
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
            Err(_) => return Err(anyhow::anyhow!("Failed to get service")),
        };

        Ok(gamepad_info)
    }
}

pub const GENERIC_ACCESS_SERVICE_UUID: &str = "00001800-0000-1000-8000-00805F9B34FB";
pub const GENERIC_ACCESS_SERVICE_CHARACTERISTIC_DEVICE_NAME_UUID: &str = "00002A00-0000-1000-8000-00805F9B34FB";
pub const GENERIC_ACCESS_SERVICE_CHARACTERISTIC_APPEARANCE_UUID: &str = "00002A01-0000-1000-8000-00805F9B34FB";
pub const GENERIC_ACCESS_SERVICE_CHARACTERISTIC_PERIPHERAL_PREFERRED_CONNECTION_PARAMETERS_UUID: &str = "00002A04-0000-1000-8000-00805F9B34FB";

pub const DEVICE_INFORMATION_SERVICE_UUID: &str = "0000180A-0000-1000-8000-00805F9B34FB";
pub const DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_MANUFACTURER_NAME_UUID: &str = "00002A29-0000-1000-8000-00805F9B34FB";
pub const DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_MODEL_NUMBER_UUID: &str = "00002A24-0000-1000-8000-00805F9B34FB";
pub const DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_SERIAL_NUMBER_UUID: &str = "00002A25-0000-1000-8000-00805F9B34FB";
pub const DEVICE_INFORMATION_SERVICE_CHARACTERISTIC_FIRMWARE_REVISION_UUID: &str = "00002A26-0000-1000-8000-00805F9B34FB";

pub const BATERY_SERVICE_UUID: &str = "0000180F-0000-1000-8000-00805F9B34FB";
pub const BATERY_SERVICE_CHARACTERISTIC_BATTERY_LEVEL_UUID: &str = "00002A19-0000-1000-8000-00805F9B34FB";
