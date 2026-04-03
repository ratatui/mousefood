# RP2040 Ratatui E-Paper Demo

This project demonstrates running a terminal UI application using `ratatui` and
`mousefood` on an [RP2040-Zero microcontroller][rp2040-zero] (by Waveshare)
, rendered onto a [Waveshare 1.54" E-Paper display][epd] over SPI.

## Pinmap

Pinout reference is based on the standard Raspberry Pi RP2040-Zero board. More
details can be found on the [official RP2040-Zero Datasheet](https://www.waveshare.com/wiki/RP2040-Zero).

![RP2040-Zero Pinout](https://www.waveshare.com/w/upload/1/1f/900px-RP2040-Zero-details-7.jpg)

### Essential Pins

| Component      | RP2040 GPIO | Description                  |
| :------------- | :---------- | :--------------------------- |
| **EPD MOSI**   | GPIO 3      | SPI Data Input (DIN)         |
| **EPD SCK**    | GPIO 2      | SPI Clock (CLK)              |
| **EPD CS**     | GPIO 5      | Chip Select (Active Low)     |
| **EPD DC**     | GPIO 6      | Data / Command Control       |
| **EPD RST**    | GPIO 10     | Reset                        |
| **EPD BUSY**   | GPIO 11     | Busy Status Output           |
| **VCC**        | 3.3V OUT    | 3.3V Power Supply            |
| **GND**        | GND         | Ground                       |

## How to Build & Flash

### 1. One-Time Setup

Install the UF2 converter tool:

```bash
cargo install elf2uf2-rs
```

Create a mount point for the RP2040:

```bash
sudo mkdir -p /mnt/rp2
```

### 2. Put the Board into BOOT Mode

The RP2040-Zero has dedicated BOOT and RESET buttons that allow you to enter
bootloader mode without unplugging the board:

1. Press and hold the **BOOT** button.
2. While holding BOOT, press and release the **RESET** button.
3. Keep holding BOOT for one more second, then release it.

The board should now appear as a USB mass storage device named **RPI-RP2**.

### 3. Build the Firmware

Navigate to the project directory and compile the code:

```bash
cargo build --release
```

The compiled binary will be located at:
`../../target/thumbv6m-none-eabi/release/rp2040-1in54-epd-example`

### 4. Convert to UF2 Format

Convert the ELF binary to UF2 format, which the RP2040 bootloader understands:

```bash
# The RP2040 bootloader only accepts UF2 files for flashing
elf2uf2-rs convert \
  ../../target/thumbv6m-none-eabi/release/rp2040-1in54-epd-example \
  flash.uf2
```

### 5. Flash the Firmware

Mount the RP2040 drive:

```bash
# Mount the RP2040 as a VFAT filesystem with synchronous writes
# The 'sync' option ensures all write operations complete immediately,
# preventing data corruption if the board reboots before buffered writes finish
sudo mount -t vfat -o sync /dev/sda1 /mnt/rp2
```

> **Note:** The device path may vary depending on your system. If you have
> multiple USB drives connected, use `lsblk` to identify the correct device
> (it may be `/dev/sdb1`, `/dev/sdc1`, etc.).

Copy the UF2 file to the mounted drive:

```bash
# The bootloader detects the file and flashes it automatically
sudo cp flash.uf2 /mnt/rp2/
```

Unmount the drive:

```bash
# Safely unmount the drive to ensure all writes are flushed
sudo umount /mnt/rp2/
```

The board will automatically reboot and start running your program.

## Notes

- **Color Translation:** Uses a custom adapter to automatically convert
  Ratatui's standard colors into the black-and-white pixels the e-paper screen
  understands.
- *Hardware Quirks & Build Requirements (`thumbv6m`)*: The RP2040-Zero lacks hardware
  atomics required by `ratatui`, so `portable-atomic` emulates them in software;
  additionally, the project must be built from its own directory to properly apply
  the `build-std` and `build-std-features` flags defined in its local `.cargo/config.toml`.
- **Memory Allocation:** A **100 KB heap** is carved out of the RP2040's RAM
  to provide sufficient space for Ratatui to build and render its UI widgets.
- **Power Efficiency:** The CPU and screen are put to sleep between frame updates
  to conserve power and prevent the e-ink display from ghosting.

[rp2040-zero]: https://www.waveshare.com/wiki/RP2040-Zero
[epd]: https://www.waveshare.com/wiki/1.54inch_e-Paper_Module_Manual
