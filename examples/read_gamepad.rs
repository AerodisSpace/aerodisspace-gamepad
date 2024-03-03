use aerodisspace_gamepad::gamepad::{ble::AerodisSpaceGamepad, gamepads::gamepads::*, gamepads::xboxone::*};

use esp_idf_hal::{delay::FreeRtos, task::block_on};
use esp_idf_svc::hal::peripherals::Peripherals;
use log::info;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    #[allow(unused)]
    let peripherals = Peripherals::take()?;

    // Nedd instance of BLEGamepad, scan and connect and get the gamepad
    let mut gamepad = AerodisSpaceGamepad::new(GamepadType::XboxOne);

    block_on(async {
        gamepad.start().await;
        info!("gamepad type: {:?}", gamepad.gamepad_type);

        while gamepad.connected() {
            let packet = gamepad.get_gamepad_packet().await;
            match packet {
                GamepadPacket::XboxOne(packet) => {
                    // info!("XboxOne packet: {:?}", packet);
                }
            }
            FreeRtos::delay_ms(1);
        }
    });

    Ok(())
}
