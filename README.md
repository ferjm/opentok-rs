# Rust bindings for OpenTok SDK
[![Rust](https://github.com/ferjm/opentok-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/ferjm/opentok-rs/actions/workflows/rust.yml)

[OpenTok SDK](https://tokbox.com/developer/sdks/linux/) bindings for Rust.

These bindings are providing a safe API that can be used to interface with OpenTok.

[Documentation](https://ferjm.github.io/opentok-rs/opentok)

## Setting up your environment
To build the OpenTok bindings or anything depending on them, you need to have
the OpenTok 2.19.1 SDK installed. For now the only supported platform is Linux and
2.19.1 is the only supported version.

You need to download the SDK from
[https://tokbox.com/developer/sdks/linux/](https://tokbox.com/developer/sdks/linux/).
Extract it somewhere in your machine and add the SDK's `lib` folder path to your
`LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables:

```sh
wget https://tokbox.com/downloads/libopentok_linux_llvm_x86_64-2.19.1
tar xvf libopentok_linux_llvm_x86_64-2.19.1 -C /home/quijote/opentok
export LD_LIBRARY_PATH="/home/quijote/opentok/libopentok_linux_llvm_x86_64-2.19.1/lib:$LD_LIBRARY_PATH"
export LIBRARY_PATH="/home/quijote/opentok/libopentok_linux_llvm_x86_64-2.19.1/lib:$LIBRARY_PATH"
```

In Fedora you need to make sure `libcxx` is installed:

```sh
sudo dnf install -y libcxx-devel
```
