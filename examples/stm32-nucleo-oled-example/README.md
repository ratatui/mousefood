# STM32 Ratatui OLED Demo

This project demonstrates running a small `ratatui` application on an
STM32 microcontroller using an **SSD1306 OLED display** over I2C.

## Pinmap

Pinout images are taken from the ARM-Mbed website <https://os.mbed.com/platforms/NUCLEO-L433RC-P/>

![left](https://os.mbed.com/media/uploads/bcostm/nucleo_l433rc_p_2018_03_12_arduino_left.png)

![right](https://os.mbed.com/media/uploads/bcostm/nucleo_l433rc_p_2018_03_12_arduino_right.png)

### Essential Pins

| Component | Pin       | Description |
| :-------- | :-------- | :---------- |
| OLED SDA  | PB7 (D14) | I2C Data    |
| OLED SCL  | PB8 (D15) | I2C Clock   |

## Notes

- The MCU runs at **80 MHz** using the internal **HSI (16 MHz)** oscillator
  routed through the PLL (Phase Locked Loop).

- The display uses the `ssd1306` crate in **buffered graphics mode**, meaning
  the framebuffer is stored in RAM and flushed to the display after each frame.

- Since `ratatui` requires dynamic allocation, a **30 KB heap** is
  configured using `embedded-alloc`.

- The Nucleo board exposes PB7 and PB8 as **Arduino pins D14 and D15**, which
  are commonly used for I2C devices.
