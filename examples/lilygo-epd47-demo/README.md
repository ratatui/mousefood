# Lilygo T5 e-paper

This example showcases the use of mouse food on the Lilygo T5 e-paper, in a `no_std` context.

## Pinmap

![./assets/T5-E-Paper-pin-en.jpg]

## Wiring

No wiring really necessary as all that's required is already on the PCB

## Notes

There are some gotchas regarding the screen refresh.

The driver either draw black on white or white on black.
Thus, superposing the updated chars which is not ideal.

To solve this you need to:

- Update the `flush_callback` to add a screen clear in it.
- Add a `terminal.clear()` to force a full redraw from mousefood.
