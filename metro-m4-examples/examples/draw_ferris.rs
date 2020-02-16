#![no_std]
#![no_main]

/// Metro M4 connected to 160x80 BGR ST7735R
/// RST connected to D1
/// DC connected to D0
/// SPI connected to Arduino 2x3 pin header

extern crate panic_halt;
extern crate metro_m4 as hal;

use embedded_graphics::image::Image16BPP;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;

use hal::spi_master;
use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{Peripherals, CorePeripherals};
use st7735_lcd;
use st7735_lcd::Orientation;

#[entry]
fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);

    let spi = spi_master(
        &mut clocks,
        16.mhz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port
    );
    
    let dc = pins.d0.into_push_pull_output(&mut pins.port);
    let rst = pins.d1.into_push_pull_output(&mut pins.port);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, true);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    let black_backdrop = Rectangle::new(Coord::new(0, 0), Coord::new(160, 80)).fill(Some(0x0000u16.into()));
    disp.set_offset(0, 25);

    disp.draw(black_backdrop.into_iter());
    
    let ferris = Image16BPP::new(include_bytes!("../../assets/ferris.raw"), 86, 64).translate(Coord::new(34, 8));
    
    disp.draw(ferris.into_iter());

    loop { continue; }
}

