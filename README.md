# st7735-lcd-examples

Usage examples for the st7735-lcd Rust crate

## Building

### Metro M4

```sh
cd metro-m4-examples
cargo build --release --examples --target=thumbv7em-none-eabihf
```

### STM32F103 Blue Pill

```sh
cd blue-pill-examples
cargo build --release --examples --target=thumbv7m-none-eabi
```

### STM32F411CE Black Pill

Build with:

```sh
cd black-pill-examples
cargo build --release --example draw_ferris
```

create a bin file and flash in DFU-Mode. To get to the DFU-Mode hold the BOOT0 button while pressing the NRST button. To flash use:

```sh
cd black-pill-examples
cargo objcopy  --release --example draw_ferris -- -O binary draw_ferris.bin
sudo dfu-util -d 0483:df11 -a 0 -s 0x08000000:leave -D draw_ferris.bin
```

### Nucleo F411RE

```sh
cd nucleo-411re-examples
cargo build --release --examples --target=thumbv7em-none-eabihf
```

### Raspberry Pi Pico

```sh
cd rp2040-examples
cargo build --release --example draw_ferris
```

### M5Stamp C3U Mate

```sh
cd esp32c3-examples
cargo build --release --example draw_ferris
```

## Wiring

### Metro M4

| ST7735 Pin | Metro M4 Pin       |
| ---------- | ------------------ |
| GND        | GND                |
| VCC        | 5V                 |
| SCL/SCK    | SCK (see picture)  |
| SDA/MOSI   | MOSI (see picture) |
| RES/RST    | D1                 |
| DC         | D0                 |
| CS         | D2 or GND          |
| BLK        | Not connected      |

<img src="https://cdn-learn.adafruit.com/assets/assets/000/069/241/medium640/adafruit_products_Grand_Central_SPI_Header_Pinout.jpg?1547248943" width="200"/>

### Blue Pill

| ST7735 Pin | Blue Pill Pin |
| ---------- | ------------- |
| GND        | G             |
| VCC        | 5V            |
| SCL/SCK    | A5            |
| SDA/MOSI   | A7            |
| RES/RST    | B1            |
| DC         | B0            |
| CS         | G             |
| BLK        | Not connected |

### Black Pill

![Black Pill connections](black-pill-examples/img/blackpill-fritzing.png)

| ST7735 Pin   | Black Pill Pin |
| ------------ | -------------- |
| GND          | G              |
| VCC          | 5V             |
| SCL/SCK      | A5             |
| SDA/MOSI/DIN | A7             |
| RES/RST      | B1             |
| DC           | B0             |
| CS           | G or PA4       |
| BL           | 3V3            |

### Nucleo F411RE

| ST7735 Pin | Nucleo F411RE Pin |
| ---------- | ----------------- |
| GND        | GND               |
| VCC        | 5V                |
| SCL/SCK    | SCK/D13           |
| SDA/MOSI   | PWM/MOSI/D11      |
| RES/RST    | A1                |
| DC         | A0                |
| CS         | GND               |
| BLK        | Not connected     |

### Raspberry Pi Pico

| ST7735 Pin | Raspberry Pi Pico Pin |
| ---------- | --------------------- |
| GND        | GND                   |
| VCC        | 5V                    |
| SCL/SCK    | gpio6                 |
| SDA/MOSI   | gpio7                 |
| RES/RST    | gpio14                |
| DC         | gpio13                |
| CS         | GND                   |
| LED        | gpio12                |

### M5Stamp C3U

| ST7735R Pin | M5Stamp C3U Pin |
| ----------- | --------------- |
| GND         | GND             |
| 3V3         | 3V3             |
| SCL/SCK     | gpio8           |
| SDA/MOSI/SI | gpio10          |
| RES/RST     | gpio3           |
| DC          | gpio4           |
| TCS/CS      | GND             |
| LED         | Not connected   |
