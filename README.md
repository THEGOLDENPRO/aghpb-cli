<div align="center">

  # 💽 aghpb-cli

  <sub>Fresh anime girls holding programming books right from the comfort of your command line.</sub>

  [[Preview Video]](https://github.com/THEGOLDENPRO/aghpb-cli/assets/66202304/8a153986-0ed2-4b6d-92a0-a8729faa0d7d)

  [[Preview Video 2]](https://github.com/THEGOLDENPRO/aghpb-cli/assets/66202304/f55176d1-b43c-4e15-97a1-220bc1f3ce39)

</div>

> [!Note]
> 
> This project is a work in progress so expect bugs and missing features. (feel free to contribute and report any bugs)

## Usage 🖱️
```sh
aghpb-cli [options] {query}
```
> Here's an example with ``rust`` being the book category and [``mai``](https://myanimelist.net/character/118739/Mai_Sakurajima) the anime character:
> ```sh
> aghpb-cli -c rust mai
> ```
Check out the help command for more: ``aghpb-cli --help``

## Installation 🛠️
I don't plan on releasing to any package managers any time soon so for now you have two options, install from source (like a real man) or use my already published binary.

> [!Warning]
> On Linux if the ``aghpb-cli`` command doesn't work you may need to add the ``~/.cargo/bin`` directory to your path if you . You can do so by adding ``export PATH=$PATH:~/.cargo/bin`` to your ``.bashrc`` or an equivalent.

### Install the binary 🧑‍💻
Prerequisites: **[``wget``](https://www.gnu.org/software/wget/)**

#### Linux 🐧
```sh
wget https://github.com/THEGOLDENPRO/aghpb-cli/releases/latest/download/aghpb-cli && sudo mv ./aghpb-cli /usr/local/bin
sudo chmod +x /usr/local/bin/aghpb-cli
```

#### Windows 🪟
will add soon...

### Install from source 🏗️
Prerequisites: **[``git``](https://git-scm.com/downloads), [``rust-lang``](https://www.rust-lang.org/tools/install), ``make`` (recommended)**

```sh
git clone https://github.com/THEGOLDENPRO/aghpb-cli
cd aghpb-cli
```

#### Linux 🐧
Now if you have 'make' you may just run these commands and you're done:
```sh
make # build
make install # install to bin
```
> If you don't have 'make' for some reason go and copy the code from the [makefile](https://github.com/THEGOLDENPRO/aghpb-cli/blob/master/Makefile) yourself but try the make command at least you might already have it and I highly recommend you install it.

#### Windows 🪟
*too lazy to add the instructions, someone do it for me* 😴
like fr!
