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


```


### extra

https://github.com/DJm00n/ControllersInfo?tab=readme-ov-file