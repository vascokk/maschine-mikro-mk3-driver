# Maschine Mikro MK3 Linux Driver
Native Instruments Maschine Mikro MK3 userspace MIDI driver for Linux.

Inspired by [maschine.rs](https://github.com/wrl/maschine.rs).

## Getting Started

Let's install dependencies first:
- Debian/Ubuntu:
  ```
  sudo apt install build-essential pkg-config libasound2-dev libjack-dev libusb-1.0-0-dev libudev-dev
  ```
- Fedora/RHEL:
  ```
  sudo dnf install @development-tools alsa-lib-devel jack-audio-connection-kit-devel libusb-devel systemd-devel
  ```
- Arch Linux:
  ```
  sudo pacman -S base-devel alsa-lib pipewire-jack libusb systemd-libs  # (or `jack2` instead of `pipewire-jack`)
  ``` 

Then we can proceed with the repo:

```shell
git clone https://github.com/r00tman/maschine-mikro-mk3-driver.git; cd maschine-mikro-mk3-driver
sudo cp 98-maschine.rules /etc/udev/rules.d/
sudo udevadm control --reload && sudo udevadm trigger
cargo run --release
```

This will init the controller and create an alsaseq MIDI port called `Maschine Mikro Mk3 MIDI Out`.
Pads have been tested to work with Hydrogen, EZdrummer 2/3, Addictive Drums 2 as plugins via REAPER+LinVst and standalone via Wine.

Note that you can use your custom config with own notemappings and other settings like this:
```shell
cargo run --release -- -c example_config.toml
```

**Important note about MIDI backends:** By default, ALSA backend is used to create virtual MIDI port. If you need Jack backend, please use this command instead:
```shell
cargo run --release --features jack
```
I tried to make a version that could do both, but due to 1) how `midir` handles backends during compile-time (no features = alsa, `["jack"]` features = jack) and 2) how rust handles dependencies with different feature flag sets ([feature unification](https://github.com/rust-lang/cargo/issues/10489)), it does not seem possible.

**Note:** In previous versions, 98-maschine.rules was granting access to Maschine only to users in `input` group. This is no longer needed, the new version of the udev rules file allows Maschine to be accessed by any user. This simplifies installation, e.g., for Ubuntu users, as by default there's no `input` group there.

## Features

Everything works and is fully configurable via TOML config:
 - **Pads**: Send MIDI notes with velocity sensitivity and aftertouch
 - **All 40 Buttons**: Fully mapped to MIDI CC messages
 - **Encoder**: Rotation (relative CC), press, and touch events
 - **Touch Strip**: Mapped to MIDI CC (default: modulation wheel CC1)
 - **LEDs**: Full control with 4 brightness levels per button
 - **Screen**: Hardware support (not yet utilized)

### Advanced Features

- **Shift Combinations**: Hold Shift + any button to send different CC numbers, effectively doubling your available mappings (40 single + 40 combinations = 80 total button mappings)
- **Toggle vs Momentary Modes**: Configure any button to work as a toggle (latching) or momentary switch
- **Separate MIDI Channels**: Configure different MIDI channels for transport buttons, regular buttons, encoder, and touch strip
- **Fully Customizable CC Mappings**: Every button, encoder action, and touch strip can be mapped to any CC number (0-127)
- **Configurable Pad Notes**: Map each of the 16 pads to any MIDI note number

This driver goes beyond the official MIDI Mode capabilities. For example, unpressed pad LEDs can be completely turned off, and all buttons have 4 brightness levels instead of just Off/On.

See `example_config.toml` for a complete configuration reference with all available options.

Contributions are welcome!

## Roadmap

The initial goal of reimplementing the official MIDI Mode has been achieved. Future enhancements could include:
- GUI editor for the config file
- OSC support alongside MIDI
- Advanced modal functions similar to Maschine software (Scenes, Patterns, context-sensitive Shift+Pad actions)
