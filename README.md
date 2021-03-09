# rust-arduino-generator
Bootstrap arduino projects in rust

## Usage
1. Init a new project using this template:
   ```
   cargo generate --git https://github.com/HKalbasi/rust-arduino-generator
   ```
   [Learn how to install and more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)
2. Put your code in `src/main.rs` ([see examples](https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples))
3. Run your code with `cargo run` 🎉 (make sure you attached the arduino usb)

## Troubleshooting

### Build fails
Make sure you have installed the version of nightly compiler specified [here](https://github.com/Rahix/avr-hal).
