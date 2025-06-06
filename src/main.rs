use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use anyhow::Result;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    
    // Initialize peripherals
    let peripherals = Peripherals::take()?;
    
    // Initialize LED
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    // Blink the LED in an infinite loop
    loop {
        for pat in 10..500 {
            led.set_high()?; // Turn LED on
            FreeRtos::delay_ms(pat);

            led.set_low()?; // Turn LED off
            FreeRtos::delay_ms(pat*2);
        }

    }
}