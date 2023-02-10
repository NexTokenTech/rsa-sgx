# rsa-sgx - Rsa Crypto Algorithm Implementation For Intel® Software Guard Extensions (Intel® SGX)
This crate adapted sgx based on The Apache Milagro Cryptographic Library's amcl.

## Installation
It is necessary to have CPUs supporting Intel SGX technology for running key-holder service node.
### Enable SGX Supports on BIOS
Updated the BIOS on your machine to its latest version and enable the SGX supports on BIOS.

### Setup Intel SGX Driver and SDK
The SGX driver is embedded with Linux kernel for Ubuntu 22.04. It is suggested to build executables
on Ubuntu 22.04. Use below link to install the build dependencies and SDK.

[SGX SDK & PSW installation for Ubuntu 22.04](https://medium.com/@yangfanghao/sgx-driver-and-sdk-installation-for-ubuntu-22-04-7db6c254e65c)

### Before Testing
Before build,please make sure you installed rustup and set Cargo environment correctly.Then,It is best to execute the "source $SGX_SDK_ENVIRONMENT" command

## Testing

Unit testing can be done using cargo testing framework.

Note: `--all-features` may be replaced by `--features xx` where `xx` is
the desired feature e.g. `sgx`.

```
cargo test --all --all-features --release
```

## Features
* sgx
* std