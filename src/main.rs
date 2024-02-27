use esp_idf_hal::{delay::FreeRtos, task::block_on};
use esp_idf_svc::hal::peripherals::Peripherals;
use gamepad::{
    ble::BLEGamepad,
    gamepads::gamepads::{GamepadDevice, GamepadHandle},
};
mod gamepad;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    #[allow(unused)]
    let peripherals = Peripherals::take()?;

    // Nedd instance of BLEGamepad, scan and connect and get the gamepad
    let mut gamepad_ble = BLEGamepad::new();

    block_on(async {
        gamepad_ble.scan_and_connect().await;
        let mut gamepad = match gamepad_ble.get_gamepad().unwrap() {
            GamepadDevice::XboxOne(gamepad_xone) => gamepad_xone,
        };

        loop {
            if gamepad.connected() {
                gamepad.handle_sticks();
                gamepad.handle_buttons();
                gamepad.handle_triggers();
                gamepad.handle_battery();
                gamepad.get_device_data();
            }
            FreeRtos::delay_ms(1);
        }
    });

    Ok(())
}
