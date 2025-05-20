<div align="center">
  <img src="./grapevine.png", width="200", style="display: block; margin: 0 auto"/>

# grapevined - a music player service
</div>

## Features
- Support for playing MP3 and FLAC files (everything is resampled to 44.1khz)

- Support for .m3u playlist files
  - Disclaimer: Grapevine does NOT support extended m3u files (`#EXTM3U`), only base m3u files are supported.
- Song queue

- Looping individual songs and the entire queue

- pausing and skipping



## Installation
Currently, the only way to install Grapevine is to build from source.

**Requirements**
- go 1.24.2 or later

- python 3.2 or later (needed for `argparse`)

- if on linux, the latest version of `alsa` is needed for Oto

**Steps**
```
git clone https://github.com/altkeys/grapevined.git
cd grapevined
go build -o grapevined ./cmd/.
```
replace 3rd step with `go build -o grapevined.exe ./cmd/.` if using Windows.

After, you can execute the binary and control it using the `grapectl` script included in the root of the repository. Optionally, you can move both into `/usr/local/bin`to allow use outside of the `grapevined` directory

## grapevined usage
Grapevine is intended to be a daemon-like service, meaning you can run the `grapevined` binary and leave it running in the background (`grapevined &`) and interact with it using `grapectl`. You can also build your own custom controller service or GUI by sending commands to the TCP server hosted at `127.0.0.1:[6990-7000]` (it usually binds to 6990 first try). 

The log file `log.json` can be found at `$XDG_CONFIG_HOME/grapevined/log.json` or `~/.config/grapevined/log.json` (Unix), `$HOME/Library/Application Support/grapevined/log.json` (Darwin/Mac), and `%AppData%\grapevined\log.json` (Windows). Its contents are truncated everytime `grapevined` is ran which is probably not ideal.

## grapectl usage
```
grapectl skip
grapectl clear
grapectl pause
grapectl resume
grapectl shutdown
grapectl -l, --loop-song
grapectl -lp, --loop-playlist
grapectl add-queue <path_to_mp3_or_flac> 
grapectl add-playlist <path_to_m3u_file>
```


## Credits
The logo is inspired by Twitter's [Twemoji grape graphic](https://github.com/twitter/twemoji/blob/master/assets/72x72/1f347.png) which is licensed under [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/). The graphic includes modifications done by [milan_25](https://github.com/milan252525)

This project uses the [gopxl/beep](https://github.com/gopxl/beep) library which is licensed under the MIT License. It can be found [here](https://github.com/gopxl/beep/blob/main/LICENSE)

This project is released under the MIT License which can be found [here](https://github.com/altkeys/grapevined/blob/main/LICENSE)

## Fun Facts
- To motivate myself to finish the player, I bought 3 FLAC files from an artist I liked and did not allow myself to use a music player to play them

- The name "Grapevine" is a parody of "Widevine DRM" which did not want to work on NixOS under the Asahi Linux kernel meaning I could not listen to Spotify since it is DRM protected and it does not have an aarch64 build. I also just really love green grapes.

- the name "Grapevine" refers to the combination of `grapevined` and `grapectl` or any other controller.

