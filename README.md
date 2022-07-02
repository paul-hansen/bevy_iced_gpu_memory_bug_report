This repo replicates a bug when using [bevy_iced](https://crates.io/crates/bevy_iced) with the color picker from [iced_aw](https://crates.io/crates/iced_aw) where the gpu shared memory maxes out and crashes.

The bug is reported here: https://github.com/tasgon/bevy_iced/issues/1

Simply run `cargo run` from the project root and click "pick color" for your GPU memory to start rising.

https://user-images.githubusercontent.com/7019130/177007377-fa8b99ad-bf77-4a3d-a753-2b5fa42fb611.mp4

