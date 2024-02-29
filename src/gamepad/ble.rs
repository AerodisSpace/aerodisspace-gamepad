use anyhow::{bail, Result};
use esp32_nimble::{BLEAdvertisedDevice, BLEClient, BLEDevice};
use log::{error, info, warn};

use super::gamepads::{gamepads::GamepadDevice, xboxone::GamepadXboxOne};
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
        Ok(())
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
}
