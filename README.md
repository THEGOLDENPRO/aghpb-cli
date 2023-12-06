<div align="center">

  # aghpb-cli

  <sub>Fresh anime girls right from the comfort of your command line.</sub>

  {video goes here}

</div>

> [!WARN]
> 
> This project is a work in progress so expect bugs and missing features. (feel free to contribute)

## Usage 🖱️
```sh
aghpb-cli {query}
```

### Installation 🛠️
I don't plan on releasing to any package managers any time soon so for now you must install from source.

Prerequisites: **[``git``](https://git-scm.com/downloads), [``rust-lang``](https://www.rust-lang.org/tools/install), ``make`` (not required)**

#### Linux 🐧
```sh
git clone https://github.com/THEGOLDENPRO/aghpb-cli
cd aghpb-cli
```
Now if you have make you may just run these commands and you're done:
```sh
make # build
sudo make install # install to bin
```
If you don't have make for some reason here you go:
```sh
cargo build --release
sudo cp ./target/release/aghpb-cli /usr/local/bin
```

#### Windows 🪟
*too lazy to add the instructions, someone do it for me* 😴