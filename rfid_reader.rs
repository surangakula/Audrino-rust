#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::hal::spi::*;
use panic_halt as _;

use mfrc522::Mfrc522;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        dp.PORTD.pd0,
        dp.PORTD.pd1.into_output(&mut dp.PORTD.ddr),
        9600,
    );

    // SPI setup
    let mut cs = dp.PORTB.pb2.into_output(&mut dp.PORTB.ddr); // D10
    let spi = Spi::new(
        dp.SPI,
        dp.PORTB.pb5, // SCK
        dp.PORTB.pb3, // MOSI
        dp.PORTB.pb4, // MISO
        cs.clone(),
        Settings {
            data_order: DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver16,
            mode: Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
        },
    );

    // Reset pin (optional)
    let reset = Some(dp.PORTB.pb1.into_output(&mut dp.PORTB.ddr)); // D9

    let mut rfid = Mfrc522::new(spi, cs, reset).unwrap();

    loop {
        if let Ok(card) = rfid.reqa() {
            if let Ok(uid) = rfid.select(&card) {
                // Print UID over serial
                ufmt::uwriteln!(&mut serial, "Card UID: ").unwrap();
                for byte in uid.bytes() {
                    ufmt::uwrite!(&mut serial, "{:02X} ", byte).unwrap();
                }
                ufmt::uwriteln!(&mut serial, "\r").unwrap();

                delay.delay_ms(1000u16); // Small delay after reading
            }
        }
    }
}