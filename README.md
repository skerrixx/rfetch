<figure>
  <img width="722" height="272" alt="image" src="https://github.com/user-attachments/assets/4306121f-5f1f-4211-9c35-ba5a7d416480" />
  <figcaption><i>custom rfetch configuration</i></figcaption>
</figure>

## rfetch
### a tool for those wanting pretty screenshots and no config editing
rfetch is a tool similar to neofetch, fastfetch, screenfetch, etc. what it's built for is to provide essential system info with ascii art and a convenient beautiful format

## installation
for those using arch linux based systems, they can download it through the arch user repository (paru -S rfetch / yay -S rfetch)
for those wanting a manual install, the steps are:

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

### configuration
unlike some.. particular fetches.. rfetch is meant to be screenshot-ready without configuration, or be very easy to configure
when you first run rfetch, you'll be asked a question and a default configuration file will be created in ~/.config/rfetch/conf.jsonc
it will most likely look like this:
``` jsonc
{
        "show_beta": false,
        "color_ascii": true,
        "color_infotext": "white",
        "hide_info": [
                /*
                uncomment any string below to hide the info about it.
                beta features cannot be hidden unless you set show_beta to false
                */
                 // "headers"
                 // "packages"
                 // "os"
                 // "kernel"
                 // "uptime"
                 // "cpu"
                 // "gpu"
                 // "ram"
                 // "disk"
                 // "battery" //(only hides it if it's present at all)
        ],
        "style": "sectioned" // options: sectioned/boxed
}
```
if you came for a pretty config, here's your pretty config:
```jsonc
{
        "show_beta": true,
        "color_ascii": false,
        "color_infotext": "white",
        "hide_info": [
                 "packages",
                 "uptime",
                 "battery"
        ],
        "style": "boxed"
}
```
otherwise, here's a quick explanation of configuration options:
1. `"show_beta"` - customize if you want to show beta features(DE/WM detection
2. `"color_ascii"` - color the distro ascii art or no
3. `"color_infotext"` - you can set a custom color for the info text! (examples: "blue", "cyan", "red")
4. `"hide_info"` - select which sections of the info text to hide
5. `"style"` - two styles are available: boxed(which you saw in the photo at the top of the readme) and sectioned(the old look rfetch had before 0.7). pick whichever you like most

made with <3 by 🦀skerrixx and 🖥️francy


