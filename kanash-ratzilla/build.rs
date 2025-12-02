fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if !target.starts_with("wasm32") {
        panic!("kanash-ratzilla must be built for a wasm32 target (e.g. wasm32-unknown-unknown).");
    }
}
