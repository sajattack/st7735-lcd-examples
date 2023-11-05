#![no_std]
#![no_main]

/// Metro M4 connected to 160x80 BGR ST7735R
/// RST connected to D1
/// DC connected to D0
/// SPI connected to Arduino 2x3 pin header

use metro_m4 as bsp;
use bsp::hal as hal;

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;

use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::pac::{Peripherals, CorePeripherals};
use hal::delay::Delay;

use bsp::periph_alias;
use bsp::entry;

use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use st7735_lcd;
use st7735_lcd::Orientation;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);

    let miso = pins.miso;
    let mosi = pins.mosi;
    let sclk = pins.sclk;
    let spi_sercom = periph_alias!(peripherals.spi_sercom);
    let mclk = &mut peripherals.MCLK;

    let spi = bsp::spi_master(&mut clocks, 16u32.MHz(), spi_sercom, mclk, sclk, mosi, miso);

    let dc = pins.d0.into_push_pull_output();
    let rst = pins.d1.into_push_pull_output();

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, true, 160, 80);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.set_offset(1, 26);
    disp.clear(Rgb565::BLACK).unwrap();
    
    // draw ferris
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86);
    let image  = Image::new(&image_raw, Point::new(34, 8));
    image.draw(&mut disp).unwrap();
    rprintln!("Done Drawing");

    loop { continue; }
}

