use esp_idf_hal::task::block_on;
use esp_idf_svc::hal::peripherals::Peripherals;
mod gamepad;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    #[allow(unused)]
    let peripherals = Peripherals::take()?;

    let mut gmpd = gamepad::gamepad::Gamepad::new();
    block_on(async {
        gmpd.scan().await;
    });

    Ok(())
}
