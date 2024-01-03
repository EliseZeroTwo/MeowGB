# MeowGB

A Gameboy (DMG) emulator written in Rust, aiming for cycle accuracy and conformance with weird hardware quirks.

To view what test ROMs are currently passed and part of CI, look at [tests.md](./tests.md), test ROMs are stored in subfolders inside [test-roms/](./test-roms/) and each subfolder contains the original license of the test-roms.

## Contents

* [Features](#features)
* [Future Features](#future-features)
* [Structure](#structure)
* [Usage](#usage)
* [Key Bindings](#key-bindings)
* [Configuration](#configuration)
* [License Info](#license)

## Features

* Mostly M-cycle accurate instructions (passes tests)
* Memory bus emulation
* Pipelined CPU (passes timing tests)
* Partially working PPU (ticked at the correct speed, drawing pixel by pixel)

## Future Features

* Audio
* Visual debugger/state inspector
* T-cycle accurate PPU
* Networked link-cable

## Structure

There are currently 4 crates used in this project:

* `meowgb`: A cross-platform frontend for the emulator
* `meowgb-core`: The implementation of the emulator
* `meowgb-opcode`: Procedural macro used in `meowgb-core` for defining opcodes
* `meowgb-tests`: A frontend-less test-harness for running tests and comparing their output, used in CI for ensuring regressions are minimal and noticed as soon as possible

## Usage

You will have to build it yourself as currently there are no premade builds available, once it is in a state I am happy with I will add release builds to CI.

1. Ensure you have a Rust toolchain installed ([docs](https://www.rust-lang.org/tools/install))
2. Run `cargo install --git https://github.com/EliseZeroTwo/MeowGB.git`
3. To launch the emulator, run `meowgb --rom PATH_TO_ROM.GB`

## Key Bindings

By default the keybindings are:

| Gameboy  | Keyboard      |
|----------|---------------|
| `A`      | `A`           |
| `B`      | `S`           |
| `Start`  | `W`           |
| `Select` | `Q`           |
| `Up`     | `Arrow Up`    |
| `Down`   | `Arrow Down`  |
| `Left`   | `Arrow Left`  |
| `Right`  | `Arrow Right` |

## Configuration

Default keybindings are overridable by placing a `config.toml` either in the directory you are running the emulator from, or at `~/.meowgb/config.toml`.

An example configuration file can be found in [config.example.toml](./config.example.toml).

## License

This software is currently licensed under the [CNPLv7+](./LICENSE.md), a summary of which can be found at [here](https://thufie.lain.haus/NPL.html).
The contents of the `test-roms` folder is excluded from this, and the original license of the test ROMs are placed in their respective subdirectory, they are not compiled into any of the software, they are included in the repository purely for intergration testing in CI.

## Dependency Version Notice

The frontend uses old versions of `winit` and `egui` related libraries, this is due to incompatibility between more modern versions of the underlying crates with `egui`, `pixels`, and `winit`. In order to upgrade them, these compatibility issues need to be fixed by them.
