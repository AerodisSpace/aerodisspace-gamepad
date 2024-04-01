use std::borrow::BorrowMut;

use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    *,
};
use log::{error, info, warn};

use super::gamepads::{
    gamepads::{check_gamepad_compatibility, GamepadPacket, GamepadPacketHandler, GamepadType},
    xboxone::GamepadPacketXboxOne,
};

const PASSKEY: u32 = 1234;

pub struct AerodisSpaceGamepad<'a> {
    _ble_device: &'a mut BLEDevice,
    _ble_client: BLEClient,
    pub gamepad_type: GamepadType,
}

impl<'a> AerodisSpaceGamepad<'a> {
    pub fn new(gamepad_type: GamepadType) -> AerodisSpaceGamepad<'a> {
        let device = BLEDevice::take();
        let security = device.security();
        security.set_auth(AuthReq::all()).set_passkey(PASSKEY).resolve_rpa();

        AerodisSpaceGamepad {
            _ble_device: device,
            _ble_client: BLEClient::new(),
            gamepad_type,
        }
    }

    pub async fn start(&mut self) {
        if let Some(device) = self.scan().await {
            #[allow(unused_mut)]
            let mut client = self._ble_client.borrow_mut();
            client.on_connect(|_client| info!("Connected ✅"));
            client.on_disconnect(|_n| info!("Disconnected ❌"));
            client.on_passkey_request(|| PASSKEY);
            client.on_confirm_pin(|pin| pin == PASSKEY);

            client.connect(device.addr()).await.expect("Could not connect to gamepad");
            client.secure_connection().await.expect("SECURE CONNECTION FAILED");

            // Generic access service
            match client.get_service(uuid128!("00001800-0000-1000-8000-00805F9B34FB")).await {
                Ok(svc) => {
                    // preferred connection parameters
                    match svc.get_characteristic(uuid128!("00002A04-0000-1000-8000-00805F9B34FB")).await {
                        Ok(chr) => {
                            let values = chr.read_value().await.unwrap();
                            
                            let preferred_conn_params = (
                                u16::from_le_bytes([values[0], values[1]]), // Minimum connection interval
                                u16::from_le_bytes([values[2], values[3]]), // Maximum connection interval
                                u16::from_le_bytes([values[4], values[5]]), // Slave latency
                                u16::from_le_bytes([values[6], values[7]]), // Connection supervision timeout
                            );
                            info!("Device Preferred connection parameters: {:?}\n Updating...", preferred_conn_params);
                            
                            client
                                .update_conn_params(
                                    preferred_conn_params.0,
                                    preferred_conn_params.1,
                                    preferred_conn_params.2,
                                    preferred_conn_params.3,
                                )
                                .expect("Could not update connection parameters");
                        }
                        Err(_) => error!("Could not get characteristic GENERIC ACCESS TO UPDATE CONN PARAMS"),
                    }
                }
                Err(_) => error!("Could not get service GENERIC ACCESS TO UPDATE CONN PARAMS"),
            }
        } else {
            error!("No gamepad found!");
        }
    }

    pub fn connected(&self) -> bool {
        self._ble_client.connected()
    }

    async fn scan(&mut self) -> Option<BLEAdvertisedDevice> {
        let ble_scan = self._ble_device.get_scan();
        match ble_scan
            .active_scan(true)
            .interval(150)
            .window(150)
            .find_device(10000, |_device| check_gamepad_compatibility(&_device.name().to_string()))
            .await
        {
            Ok(device) => {
                if device.is_some() {
                    let dclone = device.clone().unwrap();
                    info!("Gamepad found: {:?} | addr: {:?}", dclone.name(), dclone.addr());
                    if dclone.name().to_string().to_lowercase().contains("xbox") {
                        self.gamepad_type = GamepadType::XboxOne
                    }
                }
                device
            }
            Err(_) => None,
        }
    }

    pub async fn get_gamepad_packet(&mut self) -> GamepadPacket {
        let svcs: Vec<&mut BLERemoteService> = self._ble_client.get_services().await.unwrap().into_iter().collect();
        match self.gamepad_type {
            GamepadType::XboxOne => {
                let mut packet = GamepadPacketXboxOne::default();
                packet.parse_packet(svcs);
                GamepadPacket::XboxOne(packet)
            }
            _ => {
                warn!("Gamepad not implemented yet!");
                GamepadPacket::XboxOne(GamepadPacketXboxOne::default())
            }
        }
    }
}
