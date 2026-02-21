# ESP32 `std` Demo

Contains examples for `std` demo using LILYGO T-Display

## Pinmap

![pinout](./assets/pin_diagram.jpg)

<br>

<details>
<summary><h4>Pin Table</h4></summary>

### **Left Header Pins** (Top to Bottom)

| Pin Label | Primary Name | Secondary Functions |
| :--- | :--- | :--- |
| **G** | GND | Ground |
| **G** | GND | Ground |
| **21** | GPIO 21 | Wire_SDA (I2C) |
| **22** | GPIO 22 | Wire_SCL (I2C) |
| **17** | GPIO 17 | |
| **2** | GPIO 2 | ADC12, TOUCH2 |
| **15** | GPIO 15 | ADC13, TOUCH3 |
| **13** | GPIO 13 | ADC14, TOUCH4 |
| **12** | GPIO 12 | ADC15, TOUCH5 |
| **G** | GND | Ground |
| **G** | GND | Ground |
| **3V** | 3V3 | Power (3.3V) |


### **Right Header Pins** (Top to Bottom)

| Pin Label | Primary Name | Secondary Functions |
| :--- | :--- | :--- |
| **3V** | 3V3 | Power (3.3V) |
| **36** | GPIO 36 | SVP, ADC0 |
| **37** | GPIO 37 | |
| **38** | GPIO 38 | |
| **39** | GPIO 39 | SVN, ADC3 |
| **32** | GPIO 32 | ADC4, TOUCH9 |
| **33** | GPIO 33 | ADC5, TOUCH8 |
| **25** | GPIO 25 | ADC18, DAC1 |
| **26** | GPIO 26 | ADC19, DAC2 |
| **27** | GPIO 27 | ADC17, TOUCH7 |
| **G** | GND | Ground |
| **5V** | 5V | Power (5V) |


### **Internal Display Pins (ST7789V)**

| Pin / GPIO | Function | Description |
| :--- | :--- | :--- |
| **19** | MOSI | SPI Data |
| **18** | SCLK | SPI Clock |
| **5** | CS | Chip Select |
| **16** | DC | Data/Command |
| **23** | RST | Reset |
| **4** | BL | Backlight |


### **Onboard Buttons**

| Button Location | Pin / Name |
| :--- | :--- |
| **Left Button** | GPIO0 |
| **Right Button** | GPIO35 |
| **Side Button** | RST (Reset) |

</details>

## Wiring

Internally the LILYGO T-Display is wired as follows 

| Component | Pin (GPIO) | Function | Direction / Type |
| :--- | :--- | :--- | :--- |
| **Display Backlight** | GPIO 4 | Backlight Control | Output |
| **Display SPI** | GPIO 18 | SCLK (SPI Clock) | Output |
| **Display SPI** | GPIO 19 | MOSI (SPI Data Out) | Output |
| **Display SPI** | GPIO 5 | CS (Chip Select) | Output |
| **Display SPI** | GPIO 16 | DC (Data/Command) | Output |
| **Display Reset** | GPIO 23 | Reset Pin (RST) | Output |
| **Button** | GPIO 0 | User Input (Triggers on Negative Edge) | Input (Interrupt) |
| **Battery Reader** | GPIO 34 | ADC Input (Measures Battery Voltage) | Analog Input (ADC1) |

<br>

## Notes

**Hardware Specifications:**
* **Board:** LILYGO TTGO T-Display V1.1 (ESP32-based)
* **Display:** 1.14-inch IPS LCD
* **Display Controller:** ST7789V
* **Resolution:** 135 x 240 pixels
* **Onboard Buttons:** 
    * Left Button: GPIO 0 (Configured for negative-edge interrupts)
    * Right Button: GPIO 35
* **Battery Monitoring:** Read via ADC1 on GPIO 34.

**Software & Driver Configurations:**
* **Framework:** Rust using `esp-idf-svc` (ESP-IDF bindings).
* **UI Library:** `ratatui` (running via the `mousefood` embedded backend).
* **SPI Settings:** 80 MHz baud rate, SPI Mode 3, Write-only (no MISO connection needed).
* **Display Driver (`mipidsi`):**
    * **Color Inversion:** Enabled (`ColorInversion::Inverted`).
    * **Hardware Offset:** Requires an X/Y offset of `(52, 40)` to render correctly on this specific panel.
    * **Orientation:** Rotated 90 degrees (`Rotation::Deg90`) for a landscape view.
* **ADC Settings (Battery):** Configured for 12-bit resolution with 11dB attenuation (`DB_11`) and Line calibration to accurately read voltages up to ~3.3V.
