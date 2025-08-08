<div align="center">
  <img src="./grapevine.png", width="200", style="display: block; margin: 0 auto"/>

# grapevined - a music player service
</div>

## Web UI & HTTP Gateway

This repo now includes:

- `gateway/` – a small Axum HTTP proxy that forwards JSON commands to the TCP daemon (ports 6990–7000)
- `web/` – a Next.js 14 (TypeScript) UI that calls `/api/*` (rewritten to the gateway via `next.config.mjs`)

## Dev quickstart

Terminal 1 (daemon):
```bash
cargo run
```

Terminal 2 (gateway):
```bash
cargo run --manifest-path gateway/Cargo.toml
```

Terminal 3 (web UI):
```bash
cd web
cp .env.local.example .env.local   # API_URL can be left at default
npm i
npm run dev
# open http://localhost:3000
```

## Notes
- The daemon remains local-only on 127.0.0.1; the gateway defaults to 127.0.0.1:8080.
- The UI calls `/api/*` which Next rewrites to `${API_URL}/api/*` (default `http://localhost:8080`).
- You can add a `STATUS` command in the daemon later; the UI already has a hook for `/api/status`.

## Features
- Support for playing MP3, FLAC, and whatever else Rodio supports without the Symphonia backend for now

- Support for .m3u playlist files
  - Disclaimer: Grapevine does NOT support extended m3u files (`#EXTM3U`), only base m3u files are supported.
- Song queue

- Looping individual songs and the entire queue

- pausing and skipping



## Installation
Currently, the only way to install Grapevine is to build from source.

**Requirements**
- Rust 1.86.0 or later

- python 3.2 or later (needed for `argparse`)

- if on linux, the latest version of your distro's `alsa` dev library that packages `alsa.pc`

**Steps**
```
git clone https://github.com/altkeys/grapevined.git
cd grapevined
cargo build --release
```

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


This project is released under the MIT License which can be found [here](https://github.com/altkeys/grapevined/blob/main/LICENSE)

## Fun Facts
- To motivate myself to finish the player, I bought 3 FLAC files from an artist I liked and did not allow myself to use a music player to play them

- The name "Grapevine" is a parody of "Widevine DRM" which did not want to work on NixOS under the Asahi Linux kernel meaning I could not listen to Spotify since it is DRM protected and it does not have an aarch64 build. I also just really love green grapes.

- The name "Grapevine" refers to the combination of `grapevined` and `grapectl` or any other controller.

- This project was rewritten from Go to Rust because of an issue within the Oto library causing insanely high 'Idle Wake Ups' and CPU usage on MacOS.

- Literally never wrote Rust before this rewrite
