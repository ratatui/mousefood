# ESP32 `std` Demo

Contains examples for `std` demo using LILYGO T-Display. This demo runs a `ratatui` interface on the board, showcasing various UI widgets (like `Charts`, `Gauge`, and `Tabs`) while reading physical button presses and monitoring the board's battery voltage.

## Pinmap

![pinout](./assets/pin_diagram.jpg)

### Essential Pins

| Component | Pin (GPIO) | Description |
| :--- | :--- | :--- |
| **Display MOSI** | 19 | SPI Data Out |
| **Display SCLK** | 18 | SPI Clock |
| **Display CS** | 5 | Chip Select |
| **Display DC** | 16 | Data/Command |
| **Display RST** | 23 | Reset Pin |
| **Display BL** | 4 | Backlight Control |
| **Left Button** | 0 | User Input |
| **Battery ADC** | 34 | Battery Voltage Reader |

## Notes

If you're adapting this demo for a different board or screen, there are some points to keep in mind :

* **Screen Configuration:** The driver (`ST7789`), screen size (`135x240`), and hardware offset (`52, 40`) are specific to the T-Display panel. You will need to adjust these values in the code to match your specific display.
* **Battery Voltage Reader:** The T-Display PCB uses a voltage divider that halves the battery voltage before it reaches the ADC on GPIO 34. This is why the ADC reading is multiplied by 2 in the code. If you are building a custom circuit, you will need to replicate this voltage divider setup to safely measure battery levels.
