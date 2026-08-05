<img width="889" height="389" alt="image" src="https://github.com/user-attachments/assets/56b4fa94-9902-4c06-82ee-fca7035f8492" />

## rfetch
### a tool for those wanting pretty screenshots and no config editing
rfetch is a tool similar to neofetch, fastfetch, screenfetch, etc. what it's built for is to provide essential system info with ascii art and a convenient beautiful format

## installation
for those using arch linux based systems, they can download it through the arch user repository.
however, the version there is currently outdated as the aur is blocking new pushes and updates.
for those wanting a (recommended) manual install, the steps are:

### dependencies
you need the basic commands for acquiring the repo. verify you have those installed using one of those commands:
```sh
sudo apt install curl git # for debian based systems

sudo pacman -S curl git # for arch based systems

sudo dnf install curl git # for fedora/rhel based
```
you will need a rust toolchain. install rustup using the command: 
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### compiling + install
run these commands in your terminal:
```sh
rustup default stable
git clone https://github.com/skerrixx/rfetch rf_compile
cd rf_compile
cargo build --release
sudo cp target/release/rfetch /usr/bin/rfetch
```

### running
you should be able to launch rfetch from your terminal of choice just by typing `rfetch`
