# rkernel

## 快速开始

环境安装直接参考：https://github.com/rcore-os/rCore-Tutorial-v3。

启动运行脚本：

```shell
# 进入内核目录
cd os

# 编译可执行内核文件：
cargo build --release

# 移除多余的元数据，生成符合qemu内存布局的可执行内核文件：
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

# 启动qemu运行rkernel内核：
qemu-system-riscv64 \
-machine virt \
-nographic \
-bios ../bootloader/rustsbi-qemu.bin \
-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

也可以使用Makefile脚本：

```shell
cd os && make run
```

示例如下图所示：

```shell
peihongchen in ~/GitHub/rkernel on main ● λ cd os
peihongchen in ~/GitHub/rkernel/os on main ● λ cargo build --release
   Compiling os v0.1.0 (/Users/peihongchen/GitHub/rkernel/os)
    Finished release [optimized] target(s) in 1.30s
peihongchen in ~/GitHub/rkernel/os on main ● λ rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
peihongchen in ~/GitHub/rkernel/os on main ● λ qemu-system-riscv64 \
-machine virt \
-nographic \
-bios ../bootloader/rustsbi-qemu.bin \
-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
[rustsbi] RustSBI version 0.2.0-alpha.6
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Implementation: RustSBI-QEMU Version 0.0.2
[rustsbi-dtb] Hart count: cluster0 with 1 cores
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: ssoft, stimer, sext (0x222)
[rustsbi] medeleg: ima, ia, bkpt, la, sa, uecall, ipage, lpage, spage (0xb1ab)
[rustsbi] pmp0: 0x10000000 ..= 0x10001fff (rwx)
[rustsbi] pmp1: 0x80000000 ..= 0x8fffffff (rwx)
[rustsbi] pmp2: 0x0 ..= 0xffffffffffffff (---)
[rustsbi] enter supervisor 0x80200000
Hello, world!
Panicked at src/main.rs:17 Shutdown machine!
peihongchen in ~/GitHub/rkernel/os on main ● λ 
```

