#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
use panic_halt as _;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use embedded_graphics::{
    pixelcolor::{raw::LittleEndian, Rgb565},
    prelude::*,
};
use hal::{
    gpio::PinState,
    spi::{Mode, Phase, Polarity},
    {pac, prelude::*},
};
use st7735_lcd::Orientation;

const SYSFREQ: u32 = 100_000_000;
const IMAGE_WIDTH: u16 = 86;
const IMAGE_HEIGHT: u16 = 64;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(SYSFREQ.Hz())
        .use_hse(25.MHz())
        .pclk1(50.MHz())
        .freeze();
    let syst = cp.SYST;
    let mut delay = syst.delay(&clocks);

    let gpioc = dp.GPIOC.split();
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    // turn on the onboard LED to show that the program is running
    gpioc.pc13.into_push_pull_output_in_state(PinState::Low);

    // SPI1
    let sck = gpioa.pa5;
    let miso = gpioa.pa6; // Not used
    let mosi = gpioa.pa7;

    let cs = gpioa.pa4.into_push_pull_output();
    let rst = gpiob.pb1.into_push_pull_output();
    let dc = gpiob.pb0.into_push_pull_output();

    let spi = dp
        .SPI1
        .spi(
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            16.MHz(),
            &clocks,
        )
        .init();

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs);

    let mut disp = st7735_lcd::ST7735::new(spi_device, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    let xpos = (160 - IMAGE_WIDTH) / 2;
    let ypos = (128 - IMAGE_HEIGHT) / 2;
    // draw ferris
    let image: embedded_graphics::image::ImageRaw<Rgb565, LittleEndian> =
        embedded_graphics::image::ImageRaw::new(
            include_bytes!("../../assets/ferris.raw"),
            IMAGE_WIDTH as u32,
        );

    disp.set_offset(xpos, ypos);
    image.draw(&mut disp).unwrap();

    loop {
        cortex_m::asm::wfi(); // sleep infinitely
    }
}
