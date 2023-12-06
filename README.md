<div align="center">

  # 💽 aghpb-cli

  <sub>Fresh anime girls holdiong programming books right from the comfort of your command line.</sub>

  [[Preview Video]](https://github.com/THEGOLDENPRO/aghpb-cli/assets/66202304/8a153986-0ed2-4b6d-92a0-a8729faa0d7d)

</div>

> [!WARNING]
> 
> This project is a work in progress so expect bugs and missing features. (feel free to contribute)

## Usage 🖱️
```sh
aghpb-cli {query}
```
Check out help for more: ``aghpb-cli --help``

## Installation 🛠️
I don't plan on releasing to any package managers any time soon so for now you must install from source.

Prerequisites: **[``git``](https://git-scm.com/downloads), [``rust-lang``](https://www.rust-lang.org/tools/install), ``make`` (not required)**

### Linux 🐧
```sh
git clone https://github.com/THEGOLDENPRO/aghpb-cli
cd aghpb-cli
```
Now if you have 'make' you may just run these commands and you're done:
```sh
make # build
sudo make install # install to bin
```
If you don't have 'make' for some reason here you go:
```sh
cargo build --release
sudo cp ./target/release/aghpb-cli /usr/local/bin
```

### Windows 🪟
*too lazy to add the instructions, someone do it for me* 😴
