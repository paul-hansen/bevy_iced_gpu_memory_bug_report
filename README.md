This repo replicates a bug when using [bevy_iced](https://crates.io/crates/bevy_iced) with the color picker from [iced_aw](https://crates.io/crates/iced_aw) where the gpu shared memory maxes out and crashes.

The bug is reported here: https://github.com/tasgon/bevy_iced/issues/1

Simply run `cargo run` from the project root and click "pick color" for your GPU memory to start rising.

./docs/bevy_iced_color_picker.mp4