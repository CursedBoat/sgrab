# SGrab
A terminal screen-capture program written in rust, powered by ffmpeg.  
Probably doesn't support wayland at all.

## Installation
1. Clone the repository
2. Run ``cargo build --release``
3. Copy ./target/release/sgrab to a convenient place
4. Add said place to path.

## Usage
1. Open SGrab by typing ``sgrab`` into your terminal, and select "Generate Config". 
2. Vim into the configs at ``~/.config/sgrab/config``, and make your desired changes.
3. Type ``sgrab -n`` to start recording your screen.

**NOTE: SGrab assumes that you have ffmpeg installed If you get any errors, please check if you have ffmpeg installed and added to path.**

## Planned features
- Add more configuration support
- Make it so that the configuration can be modified within the app
- Add more audio devices, so you can record desktop audio and your mic at the same time 
- Add cross-platform support
