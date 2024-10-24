# xv6-rust

=======
This project is a port from [xv6-risc](https://github.com/mit-pdos/xv6-riscv) to Rust.
The objective of this port to retain as much of the original code and design unless there is not a
clear way of achieving this.

## Setup
```
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
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

