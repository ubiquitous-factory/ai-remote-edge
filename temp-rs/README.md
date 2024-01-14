# temp-rs

Rust project to read temperatures from a thermoresitor on the _Arduino Mega 2560_.

There is also a complimentary Arduino script in the `arduino/temp/` folder if you want to use this project in the ML/Ops tutorial but don't want to get into the Rust aspects of the tutorial.


## schematics
![sketch](https://user-images.githubusercontent.com/5845622/74078303-197da280-4a29-11ea-867c-17422f4fbb55.png)
![schematic](https://user-images.githubusercontent.com/5845622/74078289-cb689f00-4a28-11ea-9d00-753c6b8aea1c.gif)

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
