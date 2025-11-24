#!/bin/bash
cargo build --release

arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/pong target/pong.gba
gbafix -p -thello -cHELO -mRS target/pong.gba
