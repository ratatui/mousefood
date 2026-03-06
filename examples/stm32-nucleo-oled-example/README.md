# STM32 Ratatui OLED Demo

This project demonstrates running a small `ratatui` application on an
STM32 microcontroller using an **SSD1306 OLED display** over I2C.

## Hardware Used

- **Board:** STM32 Nucleo L433RC-P  
- **Display:** 0.91" OLED (128×32) — SSD1306, I2C interface

## Pinout

Pinout images are taken from the ARM-Mbed website <https://os.mbed.com/platforms/NUCLEO-L433RC-P/>

![left](https://os.mbed.com/media/uploads/bcostm/nucleo_l433rc_p_2018_03_12_arduino_left.png)

![right](https://os.mbed.com/media/uploads/bcostm/nucleo_l433rc_p_2018_03_12_arduino_right.png)

### Essential Pins

| Component | Pin       | Description |
|:----------|:----------|:------------|
| OLED SDA  | PB7 (D14) | I2C Data    |
| OLED SCL  | PB8 (D15) | I2C Clock   |

## Notes

- **Clock Configuration:**  
  The MCU runs at **80 MHz** using the internal **HSI (16 MHz)** oscillator
routed through the PLL (Phase Locked Loop).

- **Display Driver:**  
  The display uses the `ssd1306` crate in **buffered graphics mode**, meaning
the framebuffer is stored in RAM and flushed to the display after each frame.

- **Memory Usage:**  
  Since `ratatui` requires dynamic allocation, a **30 KB heap** is
configured using `embedded-alloc`.

- **Board Mapping:**  
  The Nucleo board exposes PB7 and PB8 as **Arduino pins D14 and D15**, which
are commonly used for I2C devices.
