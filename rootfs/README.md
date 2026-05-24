# Plum RootFS

This directory is the template staged into the ISO build.

The build script copies the compiled kernel and userland binaries into `boot/` and `bin/` before invoking `xorriso`.
