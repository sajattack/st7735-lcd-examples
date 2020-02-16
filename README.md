# st7735-lcd-examples
Usage examples for the st7735-lcd Rust crate

## Building 

### Metro M4
`cargo build --release --examples --target=thumbv7em-none-eabihf`

### STM32F103 Blue Pill
`cargo build --release --examples --target=thumbv7m-none-eabi`

## Wiring

### Metro M4
| ST7735 Pin | Metro M4 Pin       |
|------------|--------------------|
| GND        | GND                |
| VCC        | 5V                 |
| SCL/SCK    | SCK (see picture)  |
| SDA/MOSI   | MOSI (see picture) |
| RES/RST    | D1                 |
| DC         | D0                 |
| CS         | GND                |
| BLK        | Not connected      |
![mosi/sck diagram](https://cdn-learn.adafruit.com/assets/assets/000/069/241/medium640/adafruit_products_Grand_Central_SPI_Header_Pinout.jpg?1547248943)

### Blue Pill
| ST7735 Pin | Blue Pill Pin |
|------------|---------------|
| GND        | G             |
| VCC        | 5V            |
| SCL/SCK    | A5            |
| SDA/MOSI   | A7            |
| RES/RST    | B1            |
| DC         | B0            |
| CS         | G             |
| BLK        | Not connected |
