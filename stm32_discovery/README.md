# Dependencies
Needs, 
gcc-arm-embedded
make
stlink
openocd

rustup target add thumbv7em-none-eabihf

# Startup
cargo build
openocd -f board/stm32f3discovery.cfg
arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/stm32_discovery
load
monitor reset init
break stm32_discovery::main
continue

