#!/bin/bash
#
#xargo build -Z build-std --target arm-none-eabihf 
cargo build \
    --target arm-none-eabihf.json \
    -Z build-std=core,compiler_builtins \
    --crate-type=staticlib
sleep 1
arm-none-eabi-gcc -g -mcpu=arm1176jzf-s -fpic -ffreestanding -c boot.S -o boot.o
arm-none-eabi-gcc -g -T linker.ld -o myos.elf -ffreestanding -O2 -nostdlib boot.o $1

#arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -std=gnu99 -c kernel_test.c -o kernel.o -O2 -Wall -Wextra
#arm-none-eabi-gcc -T linker.ld -o myos.elf -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc

#arm-none-eabi-objcopy myos.elf -O binary kernel.img
