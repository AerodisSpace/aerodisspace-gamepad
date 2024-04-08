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
    let mut ble = AerodisSpaceGamepad::new(GamepadType::XboxOne);

    block_on(async {
        ble.start().await;
        info!("gamepad type: {:?}", ble.gamepad_type);
        let gamepad = ble.get_gamepad().await;
        while ble.connected() {
            
            match &gamepad {
                GamepadHandler::XboxOne(gmpd) => {
                    info!("Gamepad: {:?}", gmpd);
                },
            }
            FreeRtos::delay_ms(1);
        }
    });

    Ok(())
}
