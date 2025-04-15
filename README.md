# kanash

Learn Kana in a terminal !
See https://kana.rezoleo.fr for a demo.

![demo](./assets/demo.gif)

> [!NOTE]
> I'm pausing the ssh server part because I found a way to expose my TUI through http with [`gotty`](https://github.com/yudai/gotty)

## Installation

Download the binary from the latest  (x86_64 only for now) then make it executable with `chmod +x kanash`.

## Usage

### From the binary in the [release](https://github.com/benoitlx/kanash/releases/)

```
chmod +x kanash
./kanash
```

> [!NOTE]
> Only work on `x86_64` for now

### From docker image (Soon TM)


## TODO

- [x] Rust build and test CI
- [x] Use ttyd instead of gotty
- [ ] enum for color palette
- [ ] Add a parameter to the creation of a Kana Page (to know wheter to show hira kata or both, based on the selection in the Home Page)
- [x] Refactor the `app.rs` using the Elm architecture
    - [x] move japanese helper function to another file
    - [x] isolate the kana ui into one component
- [ ] ~~look at rust multithreading and tokio~~ (Only using `event::poll(Duration::from_millis(10)).unwrap()` in `handle_event` in order not to block the rendering)
- [x] Better UI for Kana
    - [x] tui-rain
- [x] splash screen
- [ ] add a list of unused hiragana you don't want to show
- [ ] look at how to do test with ratatui
- [ ] ~~look at https://github.com/arthepsy/ssh-audit~~ (see the first note)
