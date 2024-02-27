# AERODISSPACE-GAMEPAD

This is a library for ESP32-[XX] that allows connection to and button reading from gamepads, with a primary focus on Xbox controllers model 1780.

## Introduction
The aerodisspace-gamepad library was developed to facilitate the connection to and reading of buttons from gamepads, especially Xbox 1780 controllers, in ESP32-[XX] projects. This library is an integral part of the aerodis-space embedded project, which aims to enable a crucial part of the project, which is the connection to gamepads to control embedded devices.

## Features
Simplified connection to gamepads, especially Xbox 1780 controllers.
Reading of pressed buttons on the gamepad.
Easy integration with the aerodisspace-embedded project.

## Contributions
Contributions are welcome! Feel free to open an issue or send a pull request.

<!-- ## License -->
<!-- This library is distributed under the [License Name] License. See the LICENSE file for more details. -->

Contact
<a href="https://github.com/AerodisSpace">Check main</a>


## Extra content

<a href="https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Assigned_Numbers/out/en/Assigned_Numbers.pdf?v=1708821452195">Bluetooth® Document</a>


## Examples: 

- Read Gamepad: 
```rust
use aerodisspace_gamepad::gamepad::{
    ble::BLEGamepad,
    gamepads::gamepads::{GamepadDevice, GamepadHandle},
};
use esp_idf_hal::{delay::FreeRtos, task::block_on};
use esp_idf_svc::hal::peripherals::Peripherals;

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

```