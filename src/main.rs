use esp_idf_hal::{delay::FreeRtos, task::block_on};
use esp_idf_svc::hal::peripherals::Peripherals;
use gamepad::gamepad::Gamepad;
mod gamepad;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    #[allow(unused)]
    let peripherals = Peripherals::take()?;

    let mut gmpd = Gamepad::new();
    block_on(async {
        let _ = gmpd.start().await;

        loop {
            if gmpd.connected {
                let _ = gmpd.get_commands().await;
            } else {
                let _ = gmpd.start().await;
            }
            FreeRtos::delay_ms(1);
        }
    });

    Ok(())
}
