# Live Highlights

## Overview

A tool for creating timestamps in order to edit highlights.

The idea being that as one records themselves, potentially for a live stream, that they can press a certain button combination after a highlight worthy moment. Then, the program will write down what time frame that happened in: currently the last sixty seconds. This can be used to more quickly edit together a highlights video of the recording.

Some future work could include the auto cutting up of the video, either with a tool like ffmpeg or through something like generating an appopriate video editor project file. TBD.

## Usage

Written in Rust with Cargo dependencies the normal way, so should be as easy as

```
cargo build
cargo run
```

## Credits

Written by Benjamin Massey, contact benjamin.w.massey@gmail.com