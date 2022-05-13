# space-orbiter
rn it just makes physically based rigid bodies and they fly around:
![image](https://user-images.githubusercontent.com/25625126/168199441-462be335-82ad-4570-9131-d867e295eeeb.png)


## How to run

To run the game, use rust 1.42 and run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
