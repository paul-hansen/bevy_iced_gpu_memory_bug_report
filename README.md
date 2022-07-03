This repo replicates a bug when using [bevy_iced](https://crates.io/crates/bevy_iced) where the gpu shared memory slowly increases over time even when showing just a simple button and eventually runs out of memory and crashes. It may take a long time to crash on GPUs with a lot of ram. I saw my memory go from 0.1 GB to 0.5 GB in about 5 minutes. It would probably take 3 ish hours to crash at that rate.

The bug is reported here: https://github.com/tasgon/bevy_iced/issues/1

Simply run `cargo run` from the project root and click "pick color" for your GPU memory to start rising.

https://user-images.githubusercontent.com/7019130/177007377-fa8b99ad-bf77-4a3d-a753-2b5fa42fb611.mp4

