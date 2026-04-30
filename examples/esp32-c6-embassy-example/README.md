# ESP32-C6 example
## Important notes
- The project scaffolding was provided by `esp-generate 1.3.0` ensuring all other esp crates are compatible if you want to add BLE etc.
- To avoid [this issue](https://github.com/ratatui/mousefood/issues/175), ensure you are using `esp-hal v1.1.0`
- It uses the `weact_studio_epd`
- Uses probe-rs for flashing
## What this demo does

Two async tasks run concurrently:

printer_task — cycles through a list of greeting messages every 5 seconds, storing the current index in an atomic
main loop — polls the atomic every 15 seconds and triggers a full e-paper refresh, rendering the current message into a bordered Ratatui widget

All logs are emitted via defmt over RTT.

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
    

This could be extended to display sensor data and publish over bluetooth etc. Ideally the display should be moved into an async task for displays that dont require a flush callback.
