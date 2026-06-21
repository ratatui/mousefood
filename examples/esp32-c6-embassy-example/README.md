# ESP32-C6 Embassy EPD Demo

This project demonstrates running a small `ratatui` application on an
ESP32-C6 microcontroller using a **WeAct Studio 2.13" e-paper display** over
SPI.

The demo runs two async tasks concurrently:

- `printer_task` cycles through greeting messages every 5 seconds and stores
  the current index in an atomic.
- The main loop polls the atomic every 15 seconds, triggers a full e-paper
  refresh, and renders the current message into a bordered Ratatui widget.

```sh
[INFO ] Booting...
[INFO ] Embassy initialized!
[INFO ] EPD driver initialized
[INFO ] Initial draw complete
[INFO ] Printer update: Hello World
[INFO ] Printer update: Hello Rat
[INFO ] Printer update: Hello Ferris
[INFO ] Redrawing: Hello Ferris
[INFO ] Printer update: Hello Embassy
```

<img src="esp32-c6-epd.png" alt="ESP32-C6 EPD Demo" width="400">

## Pinmap

### Essential Pins

| Component    | Pin (GPIO) | Description       |
| :----------- | :--------- | :---------------- |
| Display SCLK | 2          | SPI Clock         |
| Display MOSI | 3          | SPI Data Out      |
| Display CS   | 4          | Chip Select       |
| Display DC   | 5          | Data/Command      |
| Display RST  | 10         | Reset Pin         |
| Display BUSY | 11         | Busy Input Pullup |

## Notes

- The project scaffolding was generated with `esp-generate 1.3.0`, which keeps
  the ESP crate versions compatible if you want to add BLE or other ESP
  features.
- To avoid [ratatui/mousefood#175](https://github.com/ratatui/mousefood/issues/175),
  use `esp-hal` v1.1.0.
- The display uses the `weact-studio-epd` crate with the blocking driver.
- Logs are emitted via `defmt` over RTT.
- Flashing is done with `probe-rs`.
- This demo could be extended to display sensor data and publish it over
  Bluetooth. Ideally, display updates should move into an async task for
  displays that do not require a flush callback.
