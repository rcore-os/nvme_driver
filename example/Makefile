env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

install_qemu:
	sudo apt-get update
	sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build
	cd ~ && wget https://download.qemu.org/qemu-7.1.0.tar.xz
	cd ~ && tar xvJf qemu-7.1.0.tar.xz
	cd ~/qemu-7.0.0 && ./configure --target-list=riscv64-softmmu,riscv64-linux-user
	cd ~/qemu-7.0.0 && make -j$(nproc)
	cd ~/qemu-7.0.0 && sudo make install

build:
	cargo build --release

osbin: build
	rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/example -O binary os.bin

clean:
	cargo clean

qemu: 
	qemu-system-riscv64 \
		-machine virt \
		-serial mon:stdio \
		-bios default \
		-kernel os.bin


qemu-nvme: osbin
	qemu-system-riscv64 \
		-machine virt \
		-serial stdio \
		-display none \
		-bios default \
		-kernel os.bin \
		-smp 1 \
		-drive file=nvme.img,if=none,id=nvm \
		-device nvme,serial=deadbeef,drive=nvm \
	
# -trace "pci_*"


# -drive file=nvme.img,if=none,id=nvm,format=raw -device nvme,serial=deadbeef,drive=nvm



# qemu-system-riscv64 -smp 1 -machine virt -bios default -m 512M -no-reboot -serial mon:stdio -serial file:/tmp/serial.out -kernel target/riscv64/release/zcore.bin -initrd zCore/riscv64.img -append "LOG=warn" -drive file=nvme.img,if=none,id=nvm -device nvme,serial=xxxxx,drive=nvm

# 5.21.1.8 Interrupt Coalescing (Feature Identifier 08h)