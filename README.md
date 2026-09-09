<p align="center">
  <img width="48%" src="https://github.com/user-attachments/assets/4306121f-5f1f-4211-9c35-ba5a7d416480" alt="Custom rfetch configuration" />
  <img width="48%" src="https://github.com/user-attachments/assets/3b1136de-e4a1-43cd-8057-e22e8718cd28" alt="Custom rfetch configuration with anonymous flag and new infos" />
</p>

<p align="center">
  <i>Custom rfetch configuration</i> &nbsp;&nbsp;•&nbsp;&nbsp; <i>Anonymous flag and new system information</i>
</p>

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

### cli flags
```sh
rfetch -d arch                  # override distro art
rfetch --ascii ~/myart.txt      # custom ascii file (--ascii-path / --art work too)
rfetch -a / --anonymize         # hide username/hostname for screenshots
rfetch --json                   # machine-readable JSON (respects hide_info)
rfetch -m / --minimal           # one-line compact output (skips gpu/disk/pkgs, fast)
rfetch --no-art                 # info only, no art
rfetch --logo-only              # art only, no info
rfetch --clear-cache            # force rebuild of package cache
```

### configuration
unlike some.. particular fetches.. rfetch is meant to be screenshot-ready without configuration, or be very easy to configure
when you first run rfetch, a default configuration file will be created in ~/.config/rfetch/conf.jsonc
it will most likely look like this:
``` jsonc
{
        "color_ascii": true,
        "color_infotext": "white",
        "hide_info": [
                /*
                uncomment any string below to hide the info about it.
                */
                 // "headers"
                 // "packages"
                 // "os"
                 // "os_age"
                 // "kernel"
                 // "de/wm"
                 // "shell"
                 // "terminal"
                 // "uptime"
                 // "boot"
                 // "cpu"
                 // "gpu"
                 // "ram"
                 // "swap"
                 // "load"
                 // "processes"
                 // "disk"
                 // "battery" //(only hides it if it's present at all)
        ],
        "style": "sectioned", // options: sectioned/boxed
        "anonymize": false, // set true to hide username/hostname for screenshots
        "ascii_path": null // set to e.g. "~/.config/rfetch/ascii.txt" for custom art
}
```
if you came for a pretty config, here's your pretty config:
```jsonc
{
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
1. `"color_ascii"` - color the distro ascii art or no
2. `"color_infotext"` - you can set a custom color for the info text! (examples: "blue", "cyan", "red")
3. `"hide_info"` - select which sections of the info text to hide
4. `"style"` - two styles are available: boxed(which you saw in the photo at the top of the readme) and sectioned(the old look rfetch had before 0.7). pick whichever you like most
5. `"anonymize"` - same as `-a/--anonymize`, hides username/hostname. good for screenshots
6. `"ascii_path"` - path to a custom ascii art file (supports `~` and `$HOME`). CLI `--ascii` overrides this

made with <3 by 🦀skerrixx and ⚡francy


