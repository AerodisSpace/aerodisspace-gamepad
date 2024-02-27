#![allow(unused)]
use esp32_nimble::{
    utilities::BleUuid, BLEAdvertisedDevice, BLEClient, BLEDevice, BLERemoteCharacteristic,
};
use log::{debug, error, info, warn};

use crate::gamepad::{ble, gamepads::gamepads::check_gamepad_compatibility};

use super::gamepads::{gamepads::GamepadDevice, xboxone::GamepadXboxOne};

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

pub struct BLEGamepad<'a> {
    pub connected: bool,
    // BLE DEVICE(GAMEPAD/EXTERN)
    _gamepad_device: Option<BLEAdvertisedDevice>,
    // BLE DEVICE
    _ble_device: &'a mut BLEDevice,
    // BLE CLIENT
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

    pub async fn scan_and_connect(&mut self) -> anyhow::Result<()> {
        self.scan().await?;
        self.connect().await?;
        self.client_handles()?;
        Ok(())
    }

    /// Scan for BLE devices and return the gamepad supported devices (xboxone, ps4, etc.)
    async fn scan(&mut self) -> anyhow::Result<()> {
        info!("Scanning Gamepads...");
        let ble_scan = self._ble_device.get_scan();
        match ble_scan
            .active_scan(true)
            .interval(150)
            .window(150)
            .find_device(10000, |_device| {
                check_gamepad_compatibility(&_device.name().to_string())
            })
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
            }
        }

        Ok(())
    }

    /// Connect to the gamepad device
    async fn connect(&mut self) -> anyhow::Result<()> {
        info!("Connecting to gamepad...");
        if (self.connected) {
            info!("Already connected to gamepad");
            return Ok(());
        }
        if let Some(device) = &self._gamepad_device {
            self._ble_client.connect(device.addr()).await;
            self.connected = self._ble_client.connected();
        }

        Ok(())
    }

    fn client_handles(&mut self) -> anyhow::Result<()> {
        let device = self._gamepad_device.clone().unwrap();
        self._ble_client.on_connect(|client| {
            client.update_conn_params(120, 120, 0, 60).unwrap();
            info!("Connected to gamepad");
        });

        self._ble_client.on_disconnect(|_| {
            info!("Disconnected from gamepad");
        });
        Ok(())
    }
    /// Set the device to be used as a gamepad, xboxone, ps4, etc.
    pub fn get_gamepad(&mut self) -> anyhow::Result<GamepadDevice> {
        if self._ble_client.connected() {
            if let Some(device) = &self._gamepad_device {
                if device.name().to_string().to_lowercase().contains("xbox") {
                    Ok(GamepadDevice::XboxOne(GamepadXboxOne {
                        _ble_device: &device,
                        _ble_client: &self._ble_client,
                    }))
                } else {
                    error!("Gamepad not supported");
                    anyhow::bail!("Gamepad not supported");
                }
            } else {
                error!("No gamepad found");
                anyhow::bail!("No gamepad found");
            }
        } else {
            error!("No gamepad found");
            anyhow::bail!("No gamepad found");
        }
    }
}
