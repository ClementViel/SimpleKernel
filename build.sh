#!/bin/bash

#nasm -f elf32 kernel.asm -o kasm.o
#gcc -m32 -c kernel.c -o kc.o
#ld -m elf_i386 -T link.ld -o kernel kasm.o kc.o


arm-none-eabi-gcc -O2 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles kernel.c -o kernel.elf
arm-none-eabi-objcopy kernel.elf -O binary kernel.img
