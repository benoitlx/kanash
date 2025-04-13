# kanash

Learn Kana in a terminal !
See https://kana.rezoleo.fr for a demo.

> [!NOTE]
> I'm pausing the ssh server part because I found a way to expose my TUI through http with [`gotty`](https://github.com/yudai/gotty)



# TODO

- [ ] Rust build and test CI
- [ ] Add a parameter to the creation of a Kana Page (to know wheter to show hira kata or both, based on the selection in the Home Page)
- [x] Refactor the `app.rs` using the Elm architecture
    - [x] move japanese helper function to another file
    - [x] isolate the kana ui into one component
- [ ] look at rust multithreading and tokio
    - Once I do this I could make a tui-rain component and render it's view asynchronously from the others components
- [ ] add a list of unused hiragana you don't want to show
- [ ] look at how to do test with ratatui
- [ ] ~~look at https://github.com/arthepsy/ssh-audit~~ (see the first note)
