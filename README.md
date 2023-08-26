# Simple readonly filesystem
This filesystem is mainly designed for microcontrollers, supports no_std.
For compilation please set environment variable ROOT_FS_DIR in ./.cargo/config.toml of this crate. So likely you will need to fork the crate in order to create pathced version with your configuration

fs_example.rs demonstrates how to use the FS.
Tested on RP2040
