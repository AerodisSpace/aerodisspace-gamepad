#![allow(unused)]

use std::char;

use esp32_nimble::{
    utilities::BleUuid, BLEAdvertisedDevice, BLEClient, BLEDevice, BLERemoteCharacteristic,
};
use log::{debug, error, info, warn};

use crate::gamepad;

pub struct Gamepad<'a> {
    pub connected: bool,
    _ble_device: &'a mut BLEDevice,
    _gamepad_device: Option<BLEAdvertisedDevice>,
    _gamepad_client: Option<BLEClient>,
}

impl<'a> Gamepad<'a> {
    pub fn new() -> Gamepad<'a> {
        Gamepad {
            connected: false,
            _ble_device: BLEDevice::take(),
            _gamepad_device: None,
            _gamepad_client: None,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.scan().await?;
        if self._gamepad_device.is_some() {
            self._gamepad_client = Some(BLEClient::new());
            self.connect().await?;
        } else {
            error!("No gamepad found");
        }
        Ok(())
    }

    async fn scan(&mut self) -> anyhow::Result<()> {
        info!("Scanning Gamepads...");
        let ble_scan = self._ble_device.get_scan();

        match ble_scan
            .active_scan(true)
            .interval(150)
            .window(150)
            .find_device(10000, |_device| {
                _device.name().to_string().to_lowercase().contains("xbox")
            })
            .await
        {
            Ok(device) => {
                if let Some(device) = device {
                    info!("Gamepad found: {:?}", device);
                    self._gamepad_device = Some(device);
                } else {
                    warn!("No gamepad found");
                }
            }
            Err(_) => error!("Error scanning for gamepads"),
        }
        Ok(())
    }

    async fn connect(&mut self) -> anyhow::Result<()> {
        if self.connected {
            info!("Already connected to gamepad");
            return Ok(());
        }
        info!("Connecting to gamepad...");

        if let (Some(client), Some(device)) = (&mut self._gamepad_client, &self._gamepad_device) {
            client.on_connect(|client| {
                client.update_conn_params(120, 120, 0, 60).unwrap();
                if client.connected() {
                    info!("Connected to gamepad");
                }
            });

            client.on_disconnect(|_| {
                warn!("Disconnected from gamepad");
            });

            match client.connect(device.addr()).await {
                Ok(_) => {
                    info!("Connected to gamepad: {}", device.name().to_string());
                }
                Err(_) => {
                    error!("Error connecting to gamepad");
                }
            }

            self.connected = client.connected();
        }

        Ok(())
    }

    pub async fn get_commands(&mut self) -> anyhow::Result<()> {
        info!("Getting commands...");
        if let (Some(client), Some(device)) = (&mut self._gamepad_client, &self._gamepad_device) {
            let services = match client.get_services().await {
                Ok(service) => service,
                Err(_) => {
                    error!("Error getting gamepad service");
                    return Ok(());
                }
            };
            // TODO: Identify the correct service and characteristics for the gamepad

            info!("{:?}", services);
            for service in services {
                let characteristics = match service.get_characteristics().await {
                    Ok(characteristics) => characteristics,
                    Err(_) => {
                        error!("Error getting gamepad characteristics");
                        return Ok(());
                    }
                };
                for c in characteristics {
                    let desc = match c.get_descriptors().await {
                        Ok(desc) => desc,
                        Err(_) => {
                            error!("Error getting gamepad descriptors");
                            return Ok(());
                        }
                    };
                    for d in desc {
                        info!("{:?}", d.read_value().await);
                    }
                }
            }
        }
        Ok(())
    }
}
