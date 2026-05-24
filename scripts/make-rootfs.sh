#!/usr/bin/env bash
set -e

SIZE_MB=${1:-64}
IMAGE_PATH=${2:-"rootfs/rootfs.img"}
ROOTFS_DIR=${3:-"rootfs"}
MOUNT="/mnt/tmprootfs_plum"

echo "Creating FAT32 image: $IMAGE_PATH (${SIZE_MB} MB)"

if [ "$SIZE_MB" -lt 1 ]; then
    echo "Using 1 MB as minimum."
    SIZE_MB=1
fi

mkdir -p "$ROOTFS_DIR"

img_dir="$(dirname "$IMAGE_PATH")"
[ -n "$img_dir" ] && mkdir -p "$img_dir"

# Create empty image file
dd if=/dev/zero of="$IMAGE_PATH" bs=1M count="$SIZE_MB" status=none

# Check dependencies
if ! command -v mkfs.vfat >/dev/null 2>&1; then
    echo "mkfs.vfat not found; installing dosfstools..."
    sudo apt-get update -qq
    sudo apt-get install -y dosfstools
fi

sudo mkfs.vfat "$IMAGE_PATH"

sudo mkdir -p "$MOUNT"

cleanup() {
    if mountpoint -q "$MOUNT"; then
        sudo umount "$MOUNT"
    fi
}
trap cleanup EXIT

sudo mount -o loop,rw "$IMAGE_PATH" "$MOUNT"

sudo rm -rf "${MOUNT:?}"/*

image_basename="$(basename "$IMAGE_PATH")"

find "$ROOTFS_DIR" -maxdepth 1 -mindepth 1 | while IFS= read -r item; do
    name="$(basename "$item")"
    [ "$name" = "$image_basename" ] && continue
    sudo cp -a "$item" "$MOUNT"/
done

sync
sudo umount "$MOUNT"
trap - EXIT

echo "rootfs image created at: $IMAGE_PATH"