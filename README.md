# ov7670-i2c

A `no_std`, asynchronous Rust driver for the OV7670 camera's control registers.
It uses `embedded-hal-async` for I²C communication and exposes typed register
bitfields and helpers for settings that span multiple registers.

The driver configures the sensor over its SCCB control interface using an I²C
peripheral. Your application supplies the camera clock, manages hardware pins,
and captures pixels from the parallel data interface. This crate does not provide
frame capture or a complete sensor initialization preset.

## Basic usage

`OV7670::new` uses the default 7-bit address `0x21`. Use
`OV7670::new_with_address(i2c, address)` to supply a different address. Bus errors
are returned directly as `I2C::Error`.

`camera.reset().await?` requests a software reset. The caller must wait at least
1 ms before the next register access; the driver does not insert this delay.

## Register access

Register types, builders, enums, and the `addr` module are exported at the crate
root. Typed accessors follow the `read_*`, `write_*`, and `modify_*` naming
convention. Read-only ID registers expose typed read methods only.

Use `modify_*` to retain the other fields of an existing register. Builders
construct whole register values; their defaults are not a sensor initialization
sequence. Bitfield types also provide `from_bits` and `into_bits` conversions.

Register reads use separate I²C write and read operations rather than
`write_read`. No extra delay is inserted between them; the implementation notes
that some hardware may require one.

## Settings spanning multiple registers

The [`combined`](src/combined.rs) module groups related register values and
provides accessors for fields split across registers. Driver helpers include:

- `read_gain_v_window`, `write_gain_v_window`, and `modify_gain_v_window`.
- `read_aec_setting`, `write_aec_setting`, and `modify_aec_setting`.
- `read_h_window`, `write_h_window`, and `modify_h_window`.
- `read_scaling`, `write_scaling`, and `modify_scaling`.
- Read/write helpers for HSYNC delay, dummy lines, dummy line timing, and dummy pixels.

Combined types expose their component registers and provide `combine` and `split`
methods. Operations spanning multiple registers perform sequential bus accesses;
they are not atomic, and an error can leave a write partially applied.
