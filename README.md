# xv6-rust


## Setup
```
rustup target add riscv64gc-unknown-none-elf


```


.cargo/config.toml
```
[build]
target = "riscv64gc-unknown-none-elf"
rustflags = ['-Clink-arg=-T./kernel.ld']

[target.riscv64gc-unknown-none-elf]
linker = "riscv64-unknown-elf-ld"
runner = "qemu-system-riscv64 -machine virt -d guest_errors,unimp -smp 4 -m 128M  -serial mon:stdio -bios none -device virtio-rng-device -device virtio-gpu-device -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device -nographic -kernel "
```

