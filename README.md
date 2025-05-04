# kanash

Learn Kana in a terminal !
See https://kana.rezoleo.fr for a demo.

![demo](./assets/demo.gif)

If your goal is to learn Japanese, you should take a look at [Awesome-Japanese](https://github.com/yudataguy/Awesome-Japanese) first.

> [!NOTE]
> I'm pausing the ssh server part because I found a way to expose my TUI through http with [`ttyd`](https://github.com/tsl0922/ttyd)

## Usage

### From the binary in the [release](https://github.com/benoitlx/kanash/releases/)

```
chmod +x kanash
./kanash
```

> [!NOTE]
> Only work on `x86_64` for now

### With `cargo`

```
cargo install kanash
```

### From docker image

To expose it as a website :

> [!TIP]
> replace `./assets` with a directory containing `jpg` and `png`

```
docker run --rm -v ./assets:/home/assets -p "80:7681" blximages/kanash
```

To run it directly in your terminal

```
docker run --rm -v ./assets:/home/assets -it --entrypoint=/usr/bin/kanash blximages/kanash
```

## TODO

- [x] Rust build and test CI
- [x] Use ttyd instead of gotty
- [x] enum for color palette
- [x] Add a parameter to the creation of a Kana Page (to know wheter to show hira kata or both, based on the selection in the Home Page)
- [x] Refactor the `app.rs` using the Elm architecture
  - [x] move japanese helper function to another file
  - [x] isolate the kana ui into one component
- [ ] ~~look at rust multithreading and tokio~~ (Only using `event::poll(Duration::from_millis(10)).unwrap()` in `handle_event` in order not to block the rendering)
- [x] Better UI for Kana
  - [x] tui-rain
- [x] splash screen
- [ ] add a list of unused hiragana you don't want to show
- [ ] look at how to do test with ratatui
- [ ] take a look at ratzilla and wasm
- [ ] make a login page in order to display statistic to users
- [ ] ~~look at https://github.com/arthepsy/ssh-audit~~ (see the first note)

## Contribute

**Advices** and **PRs** are very much apreciated

## Acknowledgments

- [ratatui](https://github.com/ratatui/ratatui) :heart:
- [ttyd](https://github.com/tsl0922/ttyd)
- [wana-kana-rust](https://github.com/PSeitz/wana_kana_rust)

Also take a look at all the dependencies in [`Cargo.toml`](./Cargo.toml)
