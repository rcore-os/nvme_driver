cd example
cargo build --release
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/example -O binary os.bin
dd if=/dev/zero bs=1M count=128 of=nvme.img
make qemu-nvme
# dtc -I dtb -O dts ./1.dtb > 1.dts