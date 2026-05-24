# Plum

A small hobby operating system written in Rust.

## Creds:

- Username: `root`
- Password: `plum`

## Features

- VGA text mode
- Serial logging (COM1)
- Linear framebuffer
- BMP image rendering
- PSF bitmap fonts
- Framebuffer terminal
- PS/2 keyboard input
- Bump allocator
- Virtual memory paging
- Physical frame allocator

## Roadmap

### Boot & Memory

- [x] VGA
- [x] Serial
- [x] Framebuffer
- [x] Paging
- [x] Physical frame allocator
- [x] Heap allocator
- [ ] Slab allocator
- [ ] Memory protection
- [ ] Higher-half kernel

### Graphics

- [x] BMP rendering
- [x] PSF fonts
- [x] Framebuffer terminal
- [x] ~~PNG decoding~~ Quick OK Image
- [x] Double buffering
- [ ] Window manager
- [ ] Hardware acceleration

### Input

- [x] PS/2 keyboard
- [ ] Mouse support
- [ ] Keyboard layouts
- [ ] USB HID

### Filesystems

- [x] FAT32
- [ ] Initial ramdisk
- [x] VFS layer
- [ ] EXT2

### Processes

- [ ] Interrupt handling
- [ ] Syscalls
- [ ] User mode (Ring 3)
- [ ] Scheduler
- [ ] ELF loader
- [ ] Multitasking

### Networking

- [ ] PCI enumeration
- [ ] Network drivers
- [ ] Ethernet stack
- [ ] ARP
- [ ] IPv4
- [ ] UDP
- [ ] TCP

### Userland

- [x] Shell
- [ ] Core utilities
- [ ] Package manager
- [ ] GUI applications

## Current Status

Plum currently boots through the Rust bootloader, initializes memory management, provides a framebuffer terminal, supports PS/2 keyboard input, and can render bitmap graphics.
