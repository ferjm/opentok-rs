# Rust bindings for OpenTok SDK

[OpenTok SDK](https://tokbox.com/developer/sdks/linux/) bindings for Rust.

These bindings are providing a safe API that can be used to interface with OpenTok.

## Setting up your environment
To build the OpenTok bindings or anything depending on them, you need to have at least
the OpenTok 2.19.1 SDK installed. For now the only supported platform is Linux.
Vonage provides a Debian package that can be installed after adding their `packagecloud`
repository:

```sh
curl -s https://packagecloud.io/install/repositories/tokbox/debian/script.deb.sh | sudo bash
```

Once the repository is set up, install the `libopentok-dev` package:

```sh
sudo apt install libopentok-dev
```

For non-Debian based Linuxes, you need to download the SDK from
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
