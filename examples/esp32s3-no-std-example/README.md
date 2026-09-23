# ESP32-S3 `no_std` Example

This example runs a small Ratatui application with `mousefood` on an ESP32-S3,
rendered on a 240 × 320 ILI9341 display over SPI.

## Pinmap

| Display pin | ESP32-S3 pin | Description     |
| :---------- | :----------- | :-------------- |
| VCC         | 3V3          | Power           |
| GND         | GND          | Ground          |
| LED         | 3V3          | Backlight power |
| SCK         | GPIO12       | SPI clock       |
| SDI (MOSI)  | GPIO11       | SPI data        |
| CS          | GPIO10       | Chip select     |
| DC          | GPIO9        | Data/command    |
| RESET       | GPIO14       | Display reset   |

The display's SDO (MISO) pin is unused.

## Build and Flash

Install the Espressif Rust toolchain and flashing utility if they are not
already available:

```sh
cargo install espup espflash
espup install
```

Load the environment exported by `espup`, connect the ESP32-S3, and run the
example from this directory:

```sh
. "$HOME/export-esp.sh"
cargo run --release
```

The configured runner flashes the firmware and opens the serial monitor.

## Notes

- The display is driven at 40 MHz in SPI mode 0.
- A 256 KiB heap is used for Ratatui and Mousefood allocations.
